use colored::*;
use std::process::exit;

pub fn throw(error: &str) {
  let f_error = error.red().bold();

  println!("{f_error}");

  exit(1)
}

pub fn success(message: &str) {
  let f_msg = message.green().bold();

  println!("{f_msg}");
}

pub fn info(message: &str) {
  let f_msg = message.blue().bold();

  println!("{f_msg}");
}
