use std::process::Command;

use crate::{
  errors::{success, throw},
  files::AppFiles,
};

pub fn get_tre() {
  let files = AppFiles::new();
  let clone_dir = files.clone_dir;

  let out = Command::new("git")
    .arg("clone")
    .arg("https://github.com/HUSKI3/Tcore-Runtime-Environment")
    .arg("-b")
    .arg("release")
    .arg("--single-branch")
    .arg(clone_dir)
    .output();

  match out {
    Ok(_) => success(&format!("Cloned the repo at {}Tcore-Runtime-Environment", &AppFiles::new().clone_dir.to_str().unwrap())),
    Err(e) => throw(e.to_string().as_str()),
  }
}
