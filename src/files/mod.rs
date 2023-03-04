// We need this to keep track of all the tcli directories
// The root directory for all tcli files is $HOME/.tcli
// Components are stored in $HOME/.tcli/components/<component_name>/start.sh

use std::{
  fs::DirBuilder,
  path::PathBuf,
};

use crate::errors::{info, throw};

pub struct AppFiles {
  pub root_dir: PathBuf,
  pub components_dir: PathBuf,
  pub bin_dir : PathBuf,
  pub clone_dir : PathBuf,

  // lets start adding components
  pub tre_dir: PathBuf, // TCore Runtime Enviornment
}

impl AppFiles {
  pub fn new() -> Self {
    // get user home
    let home: PathBuf = dirs::home_dir().expect("A valid home dir could not be detected");

    // .tcli/
    let mut root_dir = home.clone();
    root_dir.push(".tcli/");

    // .tcli/bin/
    let mut bin_dir = root_dir.clone();
    bin_dir.push("bin/");

    // .tcli/components/
    let mut components_dir = root_dir.clone();
    components_dir.push("components/");

    // .tcli/git-cache/
    let mut clone_dir = root_dir.clone();
    clone_dir.push("git-cache/");

    // .tcli/components/tre/
    let mut tre_dir = components_dir.clone();
    tre_dir.push("tre/");

    return Self {
      root_dir,
      components_dir,
      bin_dir,
      clone_dir,
      tre_dir,
    };
  }
  pub fn check_and_generate(&self) {
    self.generate(&self.root_dir);
    self.generate(&self.components_dir);
    self.generate(&self.bin_dir);
    self.generate(&self.tre_dir);
  }

  fn generate(&self, path: &PathBuf) {

    if path.is_dir() {
      info(&format!(
        "{} already exists, proceeding to next step",
        path.to_str().unwrap()
      ));
      return;
    };

    let dir_builder = DirBuilder::new();
    match dir_builder.create(path) {
      Result::Ok(_) => {
        info(&format!("created path {}", path.to_str().unwrap()));
      }
      Result::Err(err) => {
        throw(err.to_string().as_str());
      }
    }
  }
}
