use crate::ui;
use termion::color;

use std::path::Path;
use std::{fs, io};

fn directory_exists(directory: &str) -> bool {
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

pub fn get_project_name() -> String {
    loop {
        let project_name = ui::input("What is your project named: ", "my-app", "my-app");

        if directory_exists(&project_name) {
            print!(
                "{}{} already exists{}",
                color::Fg(color::Red),
                project_name,
                color::Fg(color::Reset)
            );
            ui::Cursor::up(1);
            ui::Cursor::clear_line();
            ui::Cursor::beginning();
            continue;
        }
        ui::Cursor::down(1);
        ui::Cursor::clear_line();
        return project_name;
    }
}
