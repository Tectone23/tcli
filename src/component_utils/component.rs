use crate::config_utils::component_config::ComponentConfig;
use crate::config_utils::read_config;
use crate::errors::*;
use crate::files::AppFiles;
use std::fs::DirBuilder;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::{exit, Command};

pub struct ConfLoaded;
pub struct ConfDropped;

pub struct TComponent<T = ConfDropped> {
    name: String,
    pub path: PathBuf,
    state: PhantomData<T>,
    config: Option<ComponentConfig>,
}

impl TComponent<ConfDropped> {
    pub fn new(name: String) -> TComponent<ConfLoaded> {
        let files = AppFiles::new();
        let mut path = files.components_dir.clone();
        path.push(format!("{name}/"));

        let mut component = TComponent {
            name,
            path,
            state: PhantomData::<ConfLoaded>,
            config: None::<ComponentConfig>,
        };
        let has_component = component.check_path();

        if has_component {
            component.check_validity();
            component.load_config();
        } else {
            throw("Component not found");
        }

        match component.config {
            Some(_) => {
                return component;
            }
            None => {
                throw("Config could not loaded properly");
                exit(1);
            }
        };
    }

    pub fn get_new(name: String) -> TComponent<ConfLoaded> {
        let files = AppFiles::new();
        let mut path = files.components_dir.clone();
        path.push(format!("{name}/"));

        let mut component = TComponent {
            name,
            path,
            state: PhantomData::<ConfLoaded>,
            config: None::<ComponentConfig>,
        };
        let has_component = component.check_path();

        if has_component {
            component.check_validity();
            component.load_config();
            component.update();
        } else {
            info("Downloading component from remote");
            component.get_files();
        }

        component.run_install_scripts();

        match component.config {
            Some(_) => {
                return component;
            }
            None => {
                throw("Config could not loaded properly");
                exit(1);
            }
        };
    }
}

impl TComponent<ConfLoaded> {
    // Check if the path exists, if no then create it
    fn check_path(&mut self) -> bool {
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
            return false;
        }
        return true;
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

    pub fn load_config(&mut self) {
        let config = read_config(&self.path);
        self.config = Some(config);
    }

    pub fn get_config(&self) -> ComponentConfig {
        match self.config.clone() {
            Some(conf) => return conf,
            None => {
                throw("Config not loaded properly");
                exit(0);
            }
        }
    }

    fn get_files(&self) {
        let out = Command::new("git")
            .arg("clone")
            .arg(format!("https://github.com/Tectone23/{}-tcli", self.name))
            .arg("-b")
            .arg("release")
            .arg("--single-branch")
            .arg(&self.path)
            .output();

        match out {
            Ok(out) => {
                if out.status.code().unwrap() == 0 {
                    success(&format!(
                        "Cloned the repo at {}",
                        &self.path.to_str().unwrap()
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

    fn update(&self) {
        // reverting all changes made to the repo as a safety precaution
        // any errors on this stage are not critical, hence not handled
        let _ = Command::new("git")
            .arg("checkout")
            .arg(".")
            .current_dir(&self.path)
            .output();

        let out = Command::new("git")
            .arg("pull")
            .current_dir(&self.path)
            .output();

        match out {
            Ok(out) => {
                if out.status.code().unwrap() == 0 {
                    success(&format!(
                        "Updated the repo at {}",
                        &self.path.to_str().unwrap()
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

    fn run_install_scripts(&self) {
        match &self.config {
            Some(config) => {
                // println!("{:?}", config.install.cmd_nix);
                let cmd_nix = &config.install.cmd_nix;
                let home_dir = dirs::home_dir().expect("A valid home dir could not be detected");

                for x in cmd_nix {
                    let mut parts = x.splitn(2, " ");
                    let cmd = parts.next().unwrap();
                    let args = parts
                        .next()
                        .unwrap_or("")
                        .replace("$HOME", &home_dir.to_str().unwrap());

                    // println!("{cmd} ||| {args}");

                    let _ = Command::new(cmd)
                        .args(args.split(" "))
                        .current_dir(&self.path)
                        .output();

                    // println!("{:?}", out);
                }
            }
            None => throw("Config not loaded properly"),
        }
    }
}
