mod args;
mod component_utils;
mod config_utils;
mod errors;
mod files;
mod init_files;
mod utils;

use args::{ComponentList, EntityType, TcliArgs};
use clap::Parser;
use component_utils::install_runtime;
use config_utils::component_config::sample_config;
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
        sample_config();
      }
    },
  }
}
