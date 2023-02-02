use std::process::exit;
use termion::{color, style};

pub fn throw(error: &str) {
  let red = color::Fg(color::Red);
  let reset = color::Fg(color::Reset);
  let bold = style::Bold;

  println!("{red}{bold}{error}{reset}");

  exit(1)
}
