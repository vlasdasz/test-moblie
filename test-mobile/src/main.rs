use std::{
    env::current_dir,
    fs::{copy, create_dir, read_dir},
    path::Path,
};

use anyhow::Result;

fn copy_file(src: &Path, dest: &Path) -> Result<()> {
    copy(src, dest)?;
    Ok(())
}

// Function to copy a directory
fn copy_dir(src: &Path, dest: &Path) -> Result<()> {
    if !dest.exists() {
        create_dir(dest)?;
    }
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dest_path)?;
        } else {
            copy_file(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let current_dir = current_dir()?;

    dbg!(&current_dir);

    let src = Path::new("mobile-template");
    let dest = Path::new("mobile");

    copy_dir(src, dest)?;
    Ok(())
}
