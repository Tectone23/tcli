use std::process::exit;
use colored::*;

pub fn throw(error: &str) {
  let f_error = error.red().bold();

  println!("{f_error}");

  exit(1)
}
