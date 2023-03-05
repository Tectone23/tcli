use crate::config_utils::component_config::ComponentConfig;
use crate::config_utils::read_config;
use crate::errors::*;
use crate::files::AppFiles;
use std::fs::DirBuilder;
use std::path::PathBuf;
use std::process::Command;

pub struct TcliComponent {
    name: String,
    path: PathBuf,
    pub config: Option<ComponentConfig>,
}

impl TcliComponent {
    pub fn new(name: String) -> Self {
        let files = AppFiles::new();
        let mut path = files.components_dir.clone();
        path.push(format!("{name}/"));

        let mut component = TcliComponent {
            name,
            path,
            config: None::<ComponentConfig>,
        };

        // mechanisms to skip steps which are not needed - DONE
        component.check_path();
        component.get_files();
        // clean, copy and then remove cache
        component.check_validity();
        component.load_config();
        // check runner
        // load actions

        return component;
    }

    // Check if the path exists, if no then create it
    fn check_path(&mut self) {
        if !self.path.is_dir() {
            let dir_builder = DirBuilder::new();
            match dir_builder.create(&self.path) {
                Result::Ok(_) => {
                    info(&format!(
                        "created path {} for component {}",
                        &self.path.to_str().unwrap(),
                        &self.name
                    ));
                }
                Result::Err(err) => {
                    throw(err.to_string().as_str());
                }
            }
        }
    }

    fn check_validity(&self) {
        let mut config_path = self.path.clone();
        config_path.push("config.yaml");

        if !config_path.is_file() {
            throw(&format!(
                "Config file does not exist for component {}",
                self.name
            ));
        }
    }

    fn load_config(&mut self) {
        let config = read_config(&self.path);
        self.config = Some(config);
    }

    fn get_files(&self) {
        let files = AppFiles::new();
        let mut clone_dir = files.clone_dir.clone();
        clone_dir.push(&self.name);

        let out = Command::new("git")
            .arg("clone")
            .arg(format!("https://github.com/Tectone23/{}-tcli", self.name))
            .arg("-b")
            .arg("release")
            .arg("--single-branch")
            .arg(format!("{}", clone_dir.to_str().unwrap()))
            .output();

        match out {
            Ok(out) => {
                if out.status.code().unwrap() == 0 {
                    success(&format!(
                        "Cloned the repo at {}",
                        clone_dir.to_str().unwrap()
                    ));
                } else {
                    throw(&format!(
                        "GIT LOGS:\n{}",
                        String::from_utf8(out.stderr).unwrap().as_str()
                    ))
                }
            }
            Err(e) => throw(e.to_string().as_str()),
        }
    }
}
