pub mod component;

use crate::{
    errors::{success, throw},
    files::AppFiles,
};
use std::{process::Command, str::FromStr};

use self::component::TcliComponent;

pub fn install_runtime(component_name: &str) -> Option<String> {
    if cfg!(windows) {
        throw("Windows is not supported yet");
        panic!("")
    } else if cfg!(unix) {
        install_runtime_unix(component_name)
    } else {
        return Some("Target operating system is not of type windows or unix".to_string());
    }
}

fn install_runtime_unix(component_name: &str) -> Option<String> {
    let output = Command::new("python").arg("--version").output();

    match output {
        Ok(o) => {
            let mut hello = o.stdout;

            let mut count = 0;
            while count <= 6 {
                hello.remove(0);
                count = count + 1;
            }

            let major = String::from_utf8(vec![hello[0]]).expect("");
            let version = String::from_utf8(hello).expect("").replace("\n", "");

            if major == "3".to_string() {
                success(&format!(
                    "ā Python version {version} detected\nā Proceeding with installation"
                ))
            } else {
                throw(String::as_str(&format!(
          "ā Python version {version} is not supported. Please install python version 3 or above"
        )));
                return Some("Wrong python version".to_string());
            }

            // make the required files/directories
            let files = AppFiles::new();
            files.check_and_generate();

            let _tre = TcliComponent::get_new(String::from_str(component_name).unwrap());
        }
        Err(err) => {
            return Some(err.to_string());
        }
    }

    return None; // no errors occured
}
