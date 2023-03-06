pub mod component_config;

use std::fs;
use std::path::PathBuf;

use component_config::ComponentConfig;

pub fn read_config(path: &PathBuf) -> ComponentConfig {
    let mut config_path = path.clone();
    config_path.push("config.yaml");

    let config = fs::read_to_string(config_path).unwrap();

    let yaml: ComponentConfig = serde_yaml::from_str(&config).unwrap();

    return yaml;
}
