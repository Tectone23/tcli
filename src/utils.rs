use crate::errors::throw;
use crate::get_from_key;
use crate::init_files;

use init_files::{init_cogs, init_hooks};
use requestty::Question;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub fn create_project(dir: Option<String>) {

  let mut questions = vec![
    Question::input("name")
      .message("What is the name of your cog")
      .build(),
  ];

  if dir.is_none() {
    questions.push(
    Question::input("dir")
      .default(".")
      .message("Which directory do you wanna save your cog in")
      .build(),
    )
  };

  let answers = requestty::prompt(questions).unwrap();
  let cog_name = get_from_key!(answers, "name");
  let cog_dir = get_from_key!(answers, "dir");
  
  let dir = match dir {
    Some(x) => x,
    None => String::from(cog_dir),
  };

  let dir = Path::new(dir.as_str());
  let dir_str = dir.to_str().unwrap();

  if !dir.exists() {
    fs::create_dir(dir).expect(format!("Cannot create dir {dir_str}").as_str());
  }
  let is_not_empty = PathBuf::from(dir)
    .read_dir()
    .map(|mut i| i.next().is_none())
    .unwrap_or(false);
  if !is_not_empty {
    throw("The selected directory is not empty! Please choose an empty directory.");
  }

  let hooks_path = format!("{dir_str}/{cog_name}.hooks");
  let hooks_path = hooks_path.as_str();

  let cogs_path = format!("{dir_str}/{cog_name}.cog");
  let cog_path = cogs_path.as_str();

  File::create(hooks_path).expect("Cannot create hooks file");
  File::create(cog_path).expect("Cannot create cog file");
  fs::create_dir(format!("{dir_str}/src"))
    .expect(format!("Cannot create dir {dir_str}/src").as_str());

  fs::write(hooks_path, init_hooks(cog_name)).expect("Cannot write to file");
  fs::write(cog_path, init_cogs(cog_name)).expect("Cannot write to file");

  exit_cli(dir_str);

  // let license = get_from_key!(answers, "license");
}

pub fn exit_cli(dir: &str) {
  println!("\n🌟 Your new cog has been created:\n");
  println!("📁 cd {dir}\n");
  println!("🚀 Start developing in your favourite editor\n");
}

#[macro_export]
macro_rules! get_from_key {
  ($x:expr, $y:expr) => {{
    $x.get($y).unwrap().as_string().unwrap()
  }};
}
