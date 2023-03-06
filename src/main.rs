mod args;
mod component_utils;
mod config_utils;
mod errors;
mod files;
mod init_files;
mod utils;

use args::{EntityType, TcliArgs};
use clap::Parser;
use component_utils::install_runtime;
use utils::create_project;

use crate::{errors::throw, component_utils::component::TcliComponent};

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
        EntityType::ComponentGet(args) => {
            if args.component.is_empty() {
                throw("Provide a valid component name\nDo component-ls to list all available components");
            } else {
                println!("{:?}", args);
                install_runtime(&args.component[0]);
            }
        }
        EntityType::Run(args) => {
            if args.component.is_empty() {
                throw("Provide a valid component name");
            } else {
                println!("{:?}", args);
                let mut component = TcliComponent::new(args.component[0].clone());
                component.load_config();

                println!("{:?}", component.config.unwrap().actions);
            }
            
        }
    }
}
