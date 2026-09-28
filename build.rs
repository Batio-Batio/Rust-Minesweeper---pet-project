use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=assets");
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut target_dir = PathBuf::from(out_dir);
    while target_dir.file_name().and_then(|s| s.to_str()) != Some("target") {
        if !target_dir.pop() {
            return; 
        }
    }
    let profile = env::var("CONFIG_PROFILE").unwrap_or_else(|_| env::var("PROFILE").unwrap_or_default());
    if profile.is_empty() {
        return;
    }
    
    target_dir.push(profile);
    let src_assets = Path::new("assets");
    let dst_assets = target_dir.join("assets");
    
    if src_assets.exists() {
        let _ = fs::remove_dir_all(&dst_assets);
        copy_dir_all(src_assets, dst_assets).expect("Failed to copy assets folder");
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}