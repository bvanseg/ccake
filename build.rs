use std::path::Path;
use std::{fs, io};

// Copies git hooks within the hooks folder to .git/hooks.
fn main() {
    let result = copy_dir_all("hooks", ".git/hooks");

    match result {
        Ok(_) => (),
        Err(e) => println!(
            "cargo:warning=An error occurred while trying to copy git hooks: {}",
            e
        ),
    };
}

// Borrowed from https://stackoverflow.com/a/65192210
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
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
