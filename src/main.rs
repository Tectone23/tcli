mod args;
mod components;
mod errors;
mod init_files;
mod utils;
mod files;

use args::{ComponentList, EntityType, TcliArgs};
use clap::Parser;
use components::install_runtime;
use utils::create_project;

fn main() {
  // init_files();

  let args = TcliArgs::parse().sub;

  match args {
    EntityType::Init(ref dir) => {
      let directory = {
        let ref this = &dir.directory;
        match this {
          Some(x) => Some(x.clone()),
          None => None,
        }
      };
      create_project(directory);
    }
    EntityType::Add(_) => {}
    EntityType::ComponentGet(e) => match e.component {
      ComponentList::Runtime => {
        install_runtime();
      }
    },
  }
}
