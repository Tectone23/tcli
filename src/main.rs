mod args;
mod component_utils;
mod config_utils;
mod errors;
mod files;
mod init_files;
mod utils;

use std::process::Command;

use args::{Components, EntityType, TcliArgs};
use clap::Parser;
use component_utils::{
    component::{ConfLoaded, TComponent},
    install_runtime,
};
use utils::create_project;

use crate::errors::{info, throw};

const ARGUMENT_SUB: &str = "%arg%";

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
                install_runtime(&args.component[0]);
            }
        }
        EntityType::Run(args) => {
            if args.component.is_empty() {
                throw("Provide a valid component name");
            } else {
                run_component(args);
            }
        }
        EntityType::ComponentLs => list_components(),
    }
}

fn run_component(args: Components) {
    // args[0] -> Component name
    // args[1] -> Taks name
    // args[2] -> Taks arg

    let component = TComponent::new(args.component[0].clone());
    let config = component.get_config();

    let task_list = config.actions.keys();
    if args.component.len() >= 2 {
        let task = args.component[1].clone();
        let task = task.as_str();
        let task = config.actions.get(task);

        match task {
            Some(task) => {
                let mut arg = String::new();
                if task.contains(ARGUMENT_SUB) && args.component.len() >= 3 {
                    arg = args.component[2].clone();
                }
                run_component_task(task, arg, component);
            }
            None => {
                info("Task not found\nThe following tasks can be run:\n");
                for x in task_list {
                    info(&format!("    {}", x));
                }
                throw("\nNo task provided");
            }
        }
    } else {
        info("No task provided\nThe following tasks can be run:\n");
        for x in task_list {
            info(&format!("    {}", x));
        }
        throw("\nNo task provided");
    }
}

fn run_component_task(task: &str, arg: String, component: TComponent<ConfLoaded>) {
    let mut task = task.clone().to_string();
    let home_dir = dirs::home_dir().expect("A valid home dir could not be detected");

    if task.contains(ARGUMENT_SUB) {
        task = task.replace(ARGUMENT_SUB, arg.as_str())
    }

    let mut parts = task.splitn(2, " ");
    let cmd = parts.next().unwrap();
    let args = parts
        .next()
        .unwrap_or("")
        .replace("$HOME", &home_dir.to_str().unwrap())
        .replace(ARGUMENT_SUB, arg.as_str());

    let out = Command::new(cmd)
        .args(args.split(" "))
        .current_dir(component.path)
        .stdout(std::process::Stdio::inherit())
        .output();

    match out {
        Ok(_) => {
            info("Execution of component finished, exiting tcli");
        }
        Err(err) => {
            throw(&format!("{}", err));
        }
    }
}

fn list_components() {
    info("These are the available components:");
    info("");
    info("runtime - The TCore Runtime Enviornment is a portable runtime for testing Cogs without having to have full version of TCore running");
}
