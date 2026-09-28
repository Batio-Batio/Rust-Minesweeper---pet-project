## Rust Minesweeper

It's a project that I am (literally) rewriting in Rust just to

learn it. When I started this project, it was my first month

since my first try with this beautiful programming language.

So, if you are an experienced Rustacean and going to explore the

source code - you have been warned!



## Controls

* **LMB** (Left Mouse Button) - Explore a cell.

* **RMB** (Right Mouse Button) - Place a flag.

* **Smiley Face** - LMB on the smiley to refresh the field and try again.



## Config

The config file system isn't done yet. If you don't want to clear

the default 25x25 grid with 115 bombs, you can easily change the

settings in the source code before compiling, but you might need

to download the rust toolchain



Find the `field` variable in `main.rs` (or where your grid is initialized)

and edit the arguments:

```rust

let mut field = Grid::new(30.0, 25, 25, 115);

```

**Where the arguments are:**

1. `30.0` - Size of cells on a field (f32).

2. `25` - Grid width in cells (u16).

3. `25` - Grid height in cells (u16).

4. `115` - Count of bombs on a field (u16).

