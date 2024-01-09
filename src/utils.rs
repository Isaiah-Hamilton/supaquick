use crate::ui;

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

pub fn create_env(dst: &str, project_url: &str, project_anon_key: &str) -> io::Result<()> {
    fs::write(
        // TODO: this can be better
        dst.to_string() + "/.env",
        "SUPABASE_URL=".to_string() + project_url + "\nSUPABASE_ANON_KEY=" + project_anon_key,
    )?;
    Ok(())
}

pub fn get_project_name() -> String {
    let project_name = ui::input("What is your project named: ", "my-app", "my-app");

    if directory_exists(&project_name) {
        let paths = fs::read_dir(&project_name).unwrap();

        println!(
            "The directory {} contains files that could conflict:",
            project_name,
        );
        ui::Cursor::new_line();
        for path in paths {
            let display_path = path
                .as_ref()
                .unwrap()
                .path()
                .display()
                .to_string()
                .replace(&format!("{}/", project_name), "");

            if path.unwrap().path().is_dir() {
                println!("{}/", display_path);
            } else {
                println!("{}", display_path);
            }
        }
        ui::Cursor::new_line();
        println!("Either try using a new directory name, or remove the files listed above.");
        std::process::exit(0);
    }
    return project_name;
}
