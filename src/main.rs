#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::{HashMap, HashSet};
use macroquad::window::*;
use macroquad::color::{Color, WHITE};
use macroquad::texture::*;
use macroquad::input::*;
use macroquad::math::*;
use macroquad::shapes::*;
use macroquad::time::*;
use rand::seq::SliceRandom;

const DARKGRAY: Color = Color::new(0.50, 0.50, 0.50, 255.0);
const GRAY: Color = Color::new(0.65, 0.65, 0.65, 255.0);
const LIGHTGRAY: Color = Color::new(0.80, 0.80, 0.80, 255.0);

#[derive(Eq, PartialEq)]
enum State {
    Idle,
    Active,
    WinEnd,
    LoseEnd(bool)
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum GuiSprite {
    Digit(u8),
    DigitEmpty,
    DigitNegative,
    SmileIdle,
    SmileActive,
    SmileLose,
    SmileWin
}
impl GuiSprite {
    const AS_ARRAY: [Self; 16] = [Self::DigitEmpty, Self::DigitNegative,
        Self::Digit(0), Self::Digit(1), Self::Digit(2), Self::Digit(3), Self::Digit(4), Self::Digit(5), Self::Digit(6), Self::Digit(7), Self::Digit(8), Self::Digit(9),
        Self::SmileIdle, Self::SmileActive, Self::SmileLose, Self::SmileWin];
}
impl TryFrom<&char> for GuiSprite {
    type Error = char;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Digit(0)),
            '1' => Ok(Self::Digit(1)),
            '2' => Ok(Self::Digit(2)),
            '3' => Ok(Self::Digit(3)),
            '4' => Ok(Self::Digit(4)),
            '5' => Ok(Self::Digit(5)),
            '6' => Ok(Self::Digit(6)),
            '7' => Ok(Self::Digit(7)),
            '8' => Ok(Self::Digit(8)),
            '9' => Ok(Self::Digit(9)),
            ' ' => Ok(Self::DigitEmpty),
            '-' => Ok(Self::DigitNegative),
            _ => Err(*value)
        }

    }
}
impl From<&State> for GuiSprite {
    fn from(value: &State) -> Self {
        match value {
            State::Idle => Self::SmileIdle,
            State::Active => Self::SmileActive,
            State::LoseEnd(_) => Self::SmileLose,
            State::WinEnd => Self::SmileWin,
        }
    }
}
// Implementing Display for GuiSprite cause derive can't do it for me
impl std::fmt::Display for GuiSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digit(num) => write!(f, "num{num}"),
            Self::DigitEmpty => write!(f, "numN"),
            Self::DigitNegative => write!(f, "num-"),
            Self::SmileIdle => write!(f, "smile-idle"),
            Self::SmileActive => write!(f, "smile-active"),
            Self::SmileLose => write!(f, "smile-lose"),
            Self::SmileWin => write!(f, "smile-win")
        }
    }
}

struct Gui {
    bomb_x: [f32; 3],
    smile_x: f32,
    timer_x: [f32; 3],
    variations_y: (f32, f32)
}
impl Gui {
    fn new(grid: &Grid) -> Self {
        let indent_y = 7.0;
        let width = grid.width as f32 * grid.cell_size + 12.0;
        let height = 80.0;

        let elements_x = (
            width / 4.0,
            width / 2.0,
            width * 3.0 / 4.0);
        let variations_y = (
            indent_y + height / 2.0 - 25.0,
            indent_y + height / 2.0 - 20.0);

        let bomb_x: [f32; 3] = [elements_x.0 - 36.0, elements_x.0 - 12.0, elements_x.0 + 12.0];
        let smile_x: f32 = elements_x.1 - 20.0;
        let timer_x: [f32; 3] = [elements_x.2 - 36.0, elements_x.2 - 12.0, elements_x.2 + 12.0];
        Self {
            bomb_x,
            smile_x,
            timer_x,
            variations_y
        }
    }
    fn gui_elements_placement(&self, gui_sprite_sheet: &HashMap<GuiSprite, Texture2D>, sprite_sheet: &HashMap<Sprite, Texture2D>, bomb_display_data: (u16, i32), smile_state: &(State, bool), timer: u16) {
        // BOMB DISPLAY
        let mut bomb_raw: String = (bomb_display_data.0 as i32 - bomb_display_data.1).to_string();
        let mut bomb_display = [' '; 3];
        while bomb_raw.len() < 3 {
            bomb_raw.insert_str(0, " ")
        }
        for (idx, ch) in bomb_raw
            .chars()
            .skip(bomb_raw.len().saturating_sub(3))
            .enumerate() {
            bomb_display[idx] = ch
        }
        for (idx, num) in bomb_display.iter().enumerate() {
            draw_texture(&gui_sprite_sheet[&GuiSprite::try_from(num).unwrap()],
                         self.bomb_x[idx], self.variations_y.0,
                         WHITE
            )
        }
        // SMILE
        let color =
            match smile_state.1 {
            true => LIGHTGRAY,
            false => WHITE
        };
        draw_texture_ex(&sprite_sheet[&Sprite::from(smile_state.1)],
                        self.smile_x - 5.0, self.variations_y.1 - 5.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(50.0, 50.0)),
                            ..Default::default() } );
        draw_texture_ex(&gui_sprite_sheet[&GuiSprite::from(&smile_state.0)],
                     self.smile_x, self.variations_y.1,
                     color,
                     DrawTextureParams {
                         dest_size: Some(vec2(40.0, 40.0)),
                         ..Default::default() } );
        // TIMER DISPLAY
        let mut timer_raw: String = timer.to_string();
        let mut timer_display = [' '; 3];
        while timer_raw.len() < 3 {
            timer_raw.insert_str(0, " ")
        }
        for (idx, ch) in timer_raw
            .chars()
            .skip(timer_raw.len().saturating_sub(3))
            .enumerate() {
            timer_display[idx] = ch
        }
        while bomb_raw.len() < 3 {
            bomb_raw.insert_str(0, " ")
        }
        for (idx, num) in timer_display.iter().enumerate() {
            draw_texture(&gui_sprite_sheet[&GuiSprite::try_from(num).unwrap()],
                         self.timer_x[idx], self.variations_y.0,
                         WHITE
            )
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Sprite {
    Undefined,
    Sunken,
    Flag,
    MisFlag,
    Bomb,
    Fail,
    Cell(u8),
}
impl Sprite {
    const AS_ARRAY: [Self; 15] = [Self::Undefined, Self::Sunken, Self::Flag, Self::MisFlag, Self::Bomb, Self::Fail,
        Self::Cell(0), Self::Cell(1), Self::Cell(2), Self::Cell(3), Self::Cell(4), Self::Cell(5), Self::Cell(6), Self::Cell(7), Self::Cell(8)];
}
impl From<bool> for Sprite {
    fn from(value: bool) -> Self {
        if value {
            return Self::Sunken
        }
        Self::Undefined
    }
}
impl TryFrom<u16> for Sprite {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 9 {
            return Ok(Self::Cell(value as u8))
        }
        Err(value)
    }
}
// Implementing Display for Sprite cause derive can't do it for me
impl std::fmt::Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => write!(f, "undefined"),
            Self::Sunken => write!(f, "sunken"),
            Self::Flag => write!(f, "flag"),
            Self::MisFlag => write!(f, "misflag"),
            Self::Bomb => write!(f, "bomb"),
            Self::Fail => write!(f, "fail"),
            Self::Cell(number) => write!(f, "B{number}"),
        }
    }
}

#[derive(Clone)]
struct Cell {
    state: Sprite,
    bomb: bool
}
impl Cell {
    fn new() -> Self {
        Self {
            state: Sprite::Undefined,
            bomb: false
        }
    }
}

struct Grid {
    width: u16,
    height: u16,
    bomb_count: u16,
    cell_size: f32,
    body: Vec<Cell>,
    explored: HashSet<u16>,
    to_explore_buffer: Vec<u16>,
    grid_state: State,
    flag_count: i32

}
impl Grid {
    fn new(cell_size: f32, width: u16, height: u16, bomb_count: u16) -> Self {
        Self {
            width,
            height,
            bomb_count,
            cell_size,
            body: Vec::new(),
            explored: HashSet::new(),
            to_explore_buffer: Vec::new(),
            grid_state: State::Idle,
            flag_count: 0
        }
    }
    fn bomb_placement(&mut self, exclude_iter: impl Iterator<Item = u16> + Clone) {
        let mut indices: Vec<u16> = (0..self.width * self.height).collect();
        for idx in exclude_iter.clone() {
            indices.remove(idx as usize);
        }

        let mut rng = rand::rng();
        indices.shuffle(&mut rng);

        for idx in exclude_iter {
            indices.push(idx)
        }

        for &idx in indices.iter().take(self.bomb_count as usize) {
            self.body[idx as usize].bomb = true
        }
    }

    fn get_bomb_count(&self, idx: u16) -> u16 {
        let mut count = 0u16;
        for neighbor in self.get_neighbors(idx, false) {
            if self.body[neighbor as usize].bomb { count += 1 }
        }
        count
    }
    fn get_flag_count(&self, idx: u16) -> u16 {
        let mut count = 0u16;
        for neighbor in self.get_neighbors(idx, false) {
            if self.body[neighbor as usize].state == Sprite::Flag { count += 1}
        }
        count
    }
    fn get_neighbors(&self, idx: u16, key: bool) -> impl Iterator<Item = u16> + 'static + Clone {
        let mut indices = [0u16; 9];
        let mut pointer = 0usize;
        let col = (idx % self.width) as i32;
        let row = (idx / self.width) as i32;

        for vert in (-1..=1).rev() {
            for hori in (-1..=1).rev() {
                if (vert == 0 && hori == 0) && !key { continue }

                let ch_row = row + vert;
                let ch_col = col + hori;
                if ch_col >= 0 &&
                   ch_col < self.width as i32 &&
                   ch_row >= 0 &&
                   ch_row < self.height as i32 {
                    indices[pointer] = (ch_row * self.width as i32 + ch_col) as u16;
                    pointer += 1
                }
            }
        }
        indices.into_iter().take(pointer)
    }

    fn get_idx(&self, pos: (f32, f32)) -> Option<u16> {
        let indent: (f32, f32) = (6.0, 100.0);
        if pos.0 < indent.0 || pos.0 >= self.width as f32 * self.cell_size + indent.0 ||
            pos.1 < indent.1 || pos.1 >= self.height as f32 * self.cell_size + indent.1
            { return None }
        Some((pos.1 - indent.1) as u16 / self.cell_size as u16 * (self.width) + (pos.0 - indent.0) as u16 / self.cell_size as u16)
    }
    #[inline]
    fn fill(&mut self) {
        self.body.clear();
        self.body.extend(std::iter::repeat(Cell::new()).take((self.width * self.height) as usize))
    }
    #[inline]
    fn get_pos(&self, idx: u16) -> (f32, f32) {
        (
            (idx % (self.width)) as f32 * self.cell_size + 6.0,// X
            (idx / (self.width)) as f32 * self.cell_size + 100.0 // Y
        )
    }
}

fn explore(grid: &mut Grid, spark_idx: u16) {
    // Overall preset
    grid.to_explore_buffer.push(spark_idx);
    // Checking until the TOCHECK list is empty
    while !grid.to_explore_buffer.is_empty() {
        // Iteration preset
        let work_idx = grid.to_explore_buffer[0];
        grid.to_explore_buffer.swap_remove(0);
        // Add neighbors to the TOCHECK list or go to next iteration?
        if grid.body[work_idx as usize].state == Sprite::Flag { continue }
        if grid.explored.contains(&work_idx) {
            // Chording
            if grid.get_bomb_count(work_idx) == grid.get_flag_count(work_idx) {
                for idx in grid.get_neighbors(work_idx, false) {
                    if grid.body[idx as usize].state == Sprite::Flag || grid.explored.contains(&idx) { continue }
                    grid.to_explore_buffer.push(idx)
                }
            }
            // Nothing to do at next part
            continue
        }
        // Pre-push to the "already explored" list
        grid.explored.insert(work_idx);
        // Exploring a bomb?
        if grid.body[work_idx as usize].bomb {
            grid.grid_state = State::LoseEnd(false);
            grid.body[work_idx as usize].state = Sprite::Fail;
            continue
        }
        // Set cell state to similar as its bomb count around
        grid.body[work_idx as usize].state = Sprite::try_from(grid.get_bomb_count(work_idx)).unwrap();
        // Add neighbors to the TOCHECK list, if exploring a cell, that has no bombs around
        if grid.body[work_idx as usize].state == Sprite::Cell(0) {
            grid.to_explore_buffer.extend(grid.get_neighbors(work_idx, false));
        }
    }
}

// TODO! Merge decorative_gui_draw() and field_outline_draw()
// Why later: cause I'm busy coding 1.0 for the very first,
// So all the improvements - later, when there will be something to improve.
fn decorative_gui_draw(field: &Grid) {
    let outline_x_start = 3.0f32;
    let outline_y_start = 7.0f32;
    let outline_x_end = outline_x_start + field.width as f32 * field.cell_size + 3.0;
    let outline_y_end = outline_y_start + 80.0;

    // Drawing under the field a retro contrast with darker-gray and lighter-gray sides
    draw_triangle(vec2(outline_x_start, outline_y_start - 3.0),
                  vec2(outline_x_end + 3.0, outline_y_start - 3.0),
                  vec2(outline_x_start, outline_y_end + 3.0),
                  DARKGRAY);
    draw_triangle(vec2(outline_x_end + 3.0, outline_y_end + 3.0),
                  vec2(outline_x_start, outline_y_end + 3.0),
                  vec2(outline_x_end + 3.0, outline_y_start - 3.0),
                  LIGHTGRAY);
    // Fixing proportions of triangles by adding another 1:1 triangles to their problem point
    draw_triangle(vec2(outline_x_start, outline_y_end + 3.0),
                  vec2(outline_x_start, outline_y_end + 3.0 - 16.0),
                  vec2(outline_x_start + 16.0, outline_y_end + 3.0 - 16.0),
                  DARKGRAY);
    draw_triangle(vec2(outline_x_end + 3.0, outline_y_start - 3.0),
                  vec2(outline_x_end + 3.0 - 16.0, outline_y_start - 3.0),
                  vec2(outline_x_end + 3.0 - 16.0, outline_y_start - 3.0 + 16.0),
                  DARKGRAY);
    draw_triangle(vec2(outline_x_end + 3.0, outline_y_start - 3.0),
                  vec2(outline_x_end + 3.0, outline_y_start - 3.0 + 8.0),
                  vec2(outline_x_end + 3.0 - 16.0, outline_y_start - 3.0 + 16.0),
                  LIGHTGRAY);
    draw_triangle(vec2(outline_x_start, outline_y_end + 3.0),
                  vec2(outline_x_start + 16.0, outline_y_end + 3.0),
                  vec2(outline_x_start + 16.0, outline_y_end + 3.0 - 16.0),
                  LIGHTGRAY);
    // Fixing mini-corner-triangle might not be effective in rectangle straight lines
    draw_rectangle(outline_x_start, outline_y_start - 3.0,
                   outline_x_end - outline_x_start, outline_y_end - outline_y_start - 3.0,
                   DARKGRAY);
    draw_rectangle(outline_x_start + 3.0, outline_y_start + 3.0,
                   outline_x_end - outline_x_start, outline_y_end - outline_y_start,
                   LIGHTGRAY);
    // Drawing functional gui body, of course it's part of decorating
    draw_rectangle(outline_x_start + 3.0, outline_y_start,
                   outline_x_end - outline_x_start - 3.0, outline_y_end - outline_y_start,
                   GRAY);

}
fn field_outline_draw(field: &Grid) {
    let field_outline_x_start = 3.0f32;
    let field_outline_y_start = 100f32 - 3.0;
    let field_outline_x_end = field_outline_x_start + field.width as f32 * field.cell_size + 6.0;
    let field_outline_y_end = field_outline_y_start + field.height as f32 * field.cell_size + 6.0;

    // Drawing under the field a retro contrast with darker-gray and lighter-gray sides
    draw_triangle(vec2(field_outline_x_start, field_outline_y_start),
                  vec2(field_outline_x_start, field_outline_y_end),
                  vec2(field_outline_x_end, field_outline_y_start),
                  DARKGRAY);
    draw_triangle(vec2(field_outline_x_end, field_outline_y_end),
                  vec2(field_outline_x_end, field_outline_y_start),
                  vec2(field_outline_x_start, field_outline_y_end),
                  LIGHTGRAY);
    // Fixing proportions of triangles by adding another 1:1 triangles to their problem point
    draw_triangle(vec2(field_outline_x_start, field_outline_y_end),
                  vec2(field_outline_x_start, field_outline_y_end - 8.0),
                  vec2(field_outline_x_start + 8.0, field_outline_y_end - 8.0),
                  DARKGRAY);
    draw_triangle(vec2(field_outline_x_end, field_outline_y_start),
                  vec2(field_outline_x_end - 8.0, field_outline_y_start),
                  vec2(field_outline_x_end - 8.0, field_outline_y_start + 8.0),
                  DARKGRAY);
    draw_triangle(vec2(field_outline_x_end, field_outline_y_start),
                  vec2(field_outline_x_end, field_outline_y_start + 8.0),
                  vec2(field_outline_x_end - 8.0, field_outline_y_start + 8.0),
                  LIGHTGRAY);
    draw_triangle(vec2(field_outline_x_start, field_outline_y_end),
                  vec2(field_outline_x_start + 8.0, field_outline_y_end),
                  vec2(field_outline_x_start + 8.0, field_outline_y_end - 8.0),
                  LIGHTGRAY);
    // Fixing mini-corner-triangle might not be effective in rectangle straight lines
    draw_rectangle(field_outline_x_start, field_outline_y_start,
                   field_outline_x_end - field_outline_x_start - 6.0, field_outline_y_end - field_outline_y_start - 6.0,
                   DARKGRAY);
    draw_rectangle(field_outline_x_start + 6.0, field_outline_y_start + 6.0,
                   field_outline_x_end - field_outline_x_start - 6.0, field_outline_y_end - field_outline_y_start - 6.0,
                   LIGHTGRAY);
}
fn window() -> Conf {
    Conf {
        window_title: "Minesweeper".to_string(),
        window_width: 300,
        window_height: 100,
        window_resizable: false,
        ..Default::default()
    }
}

async fn load_sprites() -> HashMap<Sprite, Texture2D> {
    let keys = Vec::from(Sprite::AS_ARRAY);
    let paths: Vec<String> = Sprite::AS_ARRAY.iter().map(|name| format!("{}{}{}", "assets/textures/", name, ".png")).collect();
    let mut values: Vec<Texture2D> = Vec::with_capacity(Sprite::AS_ARRAY.len());

    for path in paths { values.push(load_texture(&path).await.unwrap()) }

    keys.iter().map(|key| key.clone()).zip(values).collect()
}
async fn load_gui_sprites() -> HashMap<GuiSprite, Texture2D> {
    let keys = Vec::from(GuiSprite::AS_ARRAY);
    let paths: Vec<String> = GuiSprite::AS_ARRAY.iter().map(|name| format!("{}{}{}", "assets/textures/", name, ".png")).collect();
    let mut values: Vec<Texture2D> = Vec::with_capacity(GuiSprite::AS_ARRAY.len());

    for path in paths { values.push(load_texture(&path).await.unwrap()) }

    keys.iter().map(|key| key.clone()).zip(values).collect()
}

#[macroquad::main(window)]
async fn main() {
    if let Ok(mut work_dir) = std::env::current_exe() {
        work_dir.pop();
        let _ = std::env::set_current_dir(&work_dir);
    }

    let mut field = Grid::new(30.0, 25, 25, 115);
    let gui = Gui::new(&field);
    let mut smile_state = (State::Idle, false);
    let sprite_sheet = load_sprites().await;
    let gui_sprite_sheet = load_gui_sprites().await;
    let mut raw_cursor_idx: Option<u16>;
    let mut cursor_idx: usize;

    let mut start_time = 0f64;
    let mut time = 0f64;
    let mut visual_buffer: Vec<u16> = Vec::new();
    field.fill();

    let (window_width, window_height) = (field.width as f32 * field.cell_size + 12.0, field.height as f32 * field.cell_size + 106.0);
    request_new_screen_size(window_width, window_height);
    next_frame().await;

    clear_background(GRAY);
    decorative_gui_draw(&field);
    field_outline_draw(&field);
    let decorative = Texture2D::from_image(&get_screen_data());

    // Game loop
    loop {
        // Preset
        if !visual_buffer.is_empty() {
            for idx in visual_buffer.iter() {
                field.body[*idx as usize].state = Sprite::Undefined
            }
            visual_buffer.clear();
        }
        if time < 1000.0 && field.grid_state == State::Active {
            time = get_time() - start_time
        }
        smile_state.1 = false;
        raw_cursor_idx = field.get_idx(mouse_position());

        // Logic START
        'check_smile: {
            if mouse_position().0 > gui.smile_x + 45.0 || mouse_position().1 > gui.variations_y.1 + 45.0 ||
                mouse_position().0 < gui.smile_x - 5.0 || mouse_position().1 < gui.variations_y.1 - 5.0 {
                break 'check_smile
            }
            if is_mouse_button_released(MouseButton::Left) && field.grid_state != State::Idle {
                start_time = 0f64;
                time = 0f64;
                visual_buffer.clear();
                field.explored.clear();
                field.grid_state = State::Idle;
                field.flag_count = 0;
                smile_state.0 = State::Idle;
                field.fill();
            }
            if is_mouse_button_down(MouseButton::Left) {
               smile_state.1 = true
            }
        }

        'mouse_button_act: {
            // -------------------- SHOULD REALLY ACCOMPLISH?
            if !matches!(field.grid_state, State::Active | State::Idle) { break 'mouse_button_act }
            smile_state.0 = State::Idle;
            if raw_cursor_idx == None { break 'mouse_button_act }
            cursor_idx = raw_cursor_idx.unwrap() as usize;

            // -------------------------------- RMB RELEASE
            if is_mouse_button_released(MouseButton::Right) {
                if field.body[cursor_idx].state == Sprite::Undefined {
                    field.body[cursor_idx].state = Sprite::Flag;
                    field.flag_count += 1
                } else if field.body[cursor_idx].state == Sprite::Flag {
                    field.body[cursor_idx].state = Sprite::Undefined;
                    field.flag_count -= 1
                }
                break 'mouse_button_act
            }
            // ------------------------------ LMB DOWN
            'cells_sunk: {
                if is_mouse_button_down(MouseButton::Left) {
                    // If cell state is flag - break
                    if matches!(field.body[cursor_idx].state, Sprite::Flag | Sprite::Cell(0)) { break 'cells_sunk }
                    // If cell under mouse isn't Undefined - pushing neighbors into visual_buffer
                    // Else - pushing the cell into vusial_buffer
                    if field.body[cursor_idx].state != Sprite::Undefined {
                        for idx in field.get_neighbors(cursor_idx as u16, false) {
                            if field.body[idx as usize].state == Sprite::Undefined {
                                visual_buffer.push(idx)
                            }
                        }
                    } else { visual_buffer.push(cursor_idx as u16) }
                    // Actually sunk all, that was in visual_buffer
                    for idx in visual_buffer.iter() {
                        field.body[*idx as usize].state = Sprite::Sunken
                    }
                    // Change smile
                    smile_state.0 = State::Active
                }
            }

            // ------------------------------- LMB RELEASE
            if is_mouse_button_released(MouseButton::Left) {
                // Check if the game not even started
                if field.grid_state == State::Idle {
                    field.grid_state = State::Active;
                    start_time = get_time();
                    field.bomb_placement(field.get_neighbors(cursor_idx as u16, true))
                }
                // Explore and auto-step
                explore(&mut field, cursor_idx as u16);
            }
        }
        'win_trigger: {
            if field.grid_state == State::WinEnd ||
                field.width * field.height > field.explored.len() as u16 + field.bomb_count {
                break 'win_trigger
            }
            field.grid_state = State::WinEnd;
            for cell in &mut field.body {
                if cell.bomb && cell.state != Sprite::Flag {
                    cell.state = Sprite::Flag;
                    field.flag_count += 1
                }
            }
            smile_state.0 = State::WinEnd
        }
        'lose_trigger: {
            if field.grid_state != State::LoseEnd(false) {
                break 'lose_trigger
            }
            field.grid_state = State::LoseEnd(true);
            for cell in &mut field.body {
                if cell.bomb && !matches!(cell.state, Sprite::Flag | Sprite::Fail) {
                    cell.state = Sprite::Bomb
                }
                if cell.state == Sprite::Flag && !cell.bomb {
                    cell.state = Sprite::MisFlag
                }
            }
            smile_state.0 = State::LoseEnd(true);
        }
        // Logic END

        // Graphics -> GUI
        clear_background(GRAY);
        draw_texture_ex(
            &decorative,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()});

        gui.gui_elements_placement(&gui_sprite_sheet, &sprite_sheet, (field.bomb_count, field.flag_count), &smile_state, time as u16);

        // Graphics -> Field
        for idx in 0..field.width * field.height {
            let (x, y) = field.get_pos(idx);
            draw_texture_ex(
                &sprite_sheet[&field.body[idx as usize].state],
                x, y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(field.cell_size, field.cell_size)),
                    ..Default::default()
                }
            )
        }
        next_frame().await
    }
}