use std::{fs, process::Command};

use crate::{
    component_utils::component::{ConfLoaded, TComponent},
    errors::{info, TcliError},
    files::AppFiles,
};

const ARGUMENT_SUB: &str = "%arg%";

pub fn run(component: &str, args: &Option<Vec<String>>) -> Result<(), TcliError> {
    let files = AppFiles::new();
    let components_dir = files.components_dir;

    let mut dir = fs::read_dir(components_dir).expect("Unable to read directory");

    let exists = dir.all(|content| {
        return component == content.unwrap().file_name().to_str().unwrap();
    });

    if !exists {
        return Err(TcliError::ComponentDoesNotExist);
    }

    let t_component = TComponent::new(component.into());
    let config = t_component.get_config();

    let notify = || {
        info("You need to specify one of the following actions to be run:");
        config.actions.iter().for_each(|(name, command)| {
            if command.contains(ARGUMENT_SUB) {
                println!("  {} [argument]", name);
            } else {
                println!("  {}", name);
            }
        });
    };

    if args.is_none() && config.actions.len() == 0 {
        let first_arg = args.as_ref().unwrap()[0].clone();
        let a = config.actions.get(&first_arg).unwrap();
        return run_component_task(a, "".into(), t_component);
    } else if args.is_none() {
        // notify of all the possible actions
        notify();
        return Ok(());
    } else {
        let first_arg = args.as_ref().unwrap()[0].clone();
        let a = config.actions.get(&first_arg).unwrap();

        if a.contains(ARGUMENT_SUB) && args.as_ref().unwrap().len() < 2 {
            notify();
            return Ok(());
        }

        return run_component_task(a, first_arg, t_component);
    }
}

fn run_component_task(
    task: &str,
    arg: String,
    component: TComponent<ConfLoaded>,
) -> Result<(), TcliError> {
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
        Ok(out) => {
            if out.status.success() {
                info("Execution of component finished, exiting tcli");
                return Ok(());
            } else {
                println!("{}", String::from_utf8_lossy(&out.stderr));
                return Err(TcliError::Other("Error while running component".into()));
            }
        }
        Err(err) => return Err(TcliError::Other(format!("{}", err))),
    }
}
