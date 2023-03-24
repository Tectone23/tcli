use std::path::PathBuf;

use crate::{cipher::encrypt_key, errors::throw, files::AppFiles};

pub fn add_user(username: String, password: String) {
    let file = check_user_exists();

    match file {
        Ok(file) => create_user(username, password, file),
        Err(err) => throw(err.as_str()),
    }
}

fn create_user(_username: String, password: String, _file: PathBuf) {
    let key = "testkeylololol";
    let a = encrypt_key(key, password.as_str());
    println!("{}", a);
}

fn check_user_exists() -> Result<PathBuf, String> {
    let files = AppFiles::new();
    let mut user_file = files.root_dir.clone();
    user_file.push("user");

    if !user_file.exists() {
        return create_user_file(user_file);
    }
    return Ok(user_file);
}

fn create_user_file(destination: PathBuf) -> Result<PathBuf, String> {
    let file = std::fs::File::create(&destination);

    match file {
        Err(_) => return Err(String::from("Failed to create a user file")),
        Ok(_) => return Ok(destination),
    }
}
