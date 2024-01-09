use std::path::Path;
use std::{fs, io};

pub fn directory_exists(directory: &str) -> bool {
    let prefixed_directory = if directory.starts_with("./") {
        directory.to_string()
    } else {
        format!("./{}", directory)
    };

    let dirs = fs::read_dir("./").unwrap();

    let dir_exists = dirs
        .filter_map(|entry| entry.ok())
        .any(|entry| entry.path().display().to_string() == prefixed_directory);

    if dir_exists {
        return true;
    }
    return false;
}

pub fn create_project(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            create_project(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
