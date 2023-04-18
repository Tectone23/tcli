mod api;
mod args;
mod cipher;
mod component_utils;
mod config_utils;
mod errors;
mod files;
mod init_files;
mod user;
mod utils;

use std::{fs, process::Command};

use args::{Components, EntityType, TcliArgs};
use clap::Parser;
use component_utils::{
    component::{ConfLoaded, TComponent},
    install_runtime,
};
use errors::success;
use requestty::{Answers, Question};
use user::{add_user, check_user};
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
        EntityType::Upload => {
            let (key, answers) = upload_cog_questions();
            let req = api::upload::RouteUpload::new();
            let res = req.request_from_answer(answers, key);
            match res {
                Ok(_) => success("Cog uploaded successfully"),
                Err(err) => {
                    throw(&format!("Failed to upload cog: {}", err));
                    unreachable!();
                }
            }
        }
        EntityType::ComponentLs => list_components(),
        EntityType::CreateUser => add_user(),
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

fn upload_cog_questions() -> (String, Answers) {
    let max_len_short_desc = 50;

    if let Ok(key) = check_user() {
        info("Uploading cog");
        let questions = vec![
            Question::input("name")
                .message("What is the name of your cog")
                .build(),
            Question::input("description")
                .message("Provide a description for your cog")
                .build(),
            Question::input("short_description")
                .message(format!("Short description (max {max_len_short_desc} char)"))
                .validate(|value, _| {
                    if value.len() > max_len_short_desc {
                        return Err(String::from(
                            "Short description must be less than 255 characters",
                        ));
                    } else {
                        return Ok(());
                    }
                })
                .validate_on_key(|value, _| {
                    if value.len() > max_len_short_desc {
                        return false;
                    } else {
                        return true;
                    }
                })
                .build(),
            Question::input("version")
                .message("Version (eg. 1 : versions are single integer)")
                .validate(|value, _| {
                    if let Ok(_) = value.parse::<i32>() {
                        return Ok(());
                    } else {
                        return Err(String::from(
                            "Version must be a valid integer (eg. 1, 2, 3)",
                        ));
                    }
                })
                .validate_on_key(|value, _| {
                    if let Ok(_) = value.parse::<i32>() {
                        return true;
                    } else {
                        return false;
                    }
                })
                .build(),
            Question::input("license")
                .message("License (eg. MIT)")
                .build(),
            Question::input("issues")
                .message("Issue tracker url (eg. https://github.com/org/repo/issues)")
                .validate(|value, _| {
                    let res = reqwest::blocking::get(value);

                    match res {
                        Ok(res) => {
                            if res.status().is_success() {
                                return Ok(());
                            } else {
                                return Err(String::from("Invalid url"));
                            }
                        }
                        Err(_) => return Err(String::from("Invalid url")),
                    }
                })
                .build(),
            Question::input("app_org")
                .message("A namespace id (eg. com.example.Cog)")
                .build(),
            Question::input("file")
                .message("The packaged cog file (eg. files-cog.zip, colour.cog)")
                .validate(|value, _| {
                    if let Ok(metadata) = fs::metadata(value) {
                        if metadata.is_file()
                            && (value.ends_with(".zip") || value.ends_with(".cog"))
                        {
                            return Ok(());
                        }
                    }
                    return Err("File does not exist or is not a zip or cog file".to_string());
                })
                .validate_on_key(|value, _| {
                    if let Ok(metadata) = fs::metadata(value) {
                        return metadata.is_file()
                            && (value.ends_with(".zip") || value.ends_with(".cog"));
                    } else {
                        return false;
                    }
                })
                .build(),
        ];
        let answers = requestty::prompt(questions);

        match answers {
            Ok(answers) => return (key, answers),
            Err(err) => {
                throw(&format!("Error when processing responses: {}", err));
                unreachable!();
            }
        }
    } else {
        throw("Passwords is incorrect");
        unreachable!();
    }
}
