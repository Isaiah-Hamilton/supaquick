use std::fs;

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

pub fn create_directory(directory: &str) -> std::io::Result<bool> {
    fs::create_dir(directory)?;
    Ok(true)
}
