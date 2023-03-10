pub mod component_config;

use crate::errors::throw;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use component_config::ComponentConfig;

pub fn read_config(path: &PathBuf) -> ComponentConfig {
    let mut config_path = path.clone();
    config_path.push("config.yaml");

    let config = fs::read_to_string(config_path);

    match config {
        Ok(config) => {
            let yaml: ComponentConfig = serde_yaml::from_str(&config).unwrap();
            return yaml;
        }
        Err(err) => {
            throw(err.to_string().as_str());
            exit(0);
        }
    }
}
