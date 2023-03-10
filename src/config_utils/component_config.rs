use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::throw;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub runner: Runners,
    pub install: Install,
    pub actions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
    pub artifacts: Vec<String>,
    pub cmd_nix: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Runners {
    #[serde(rename = "python")]
    Python
}

pub fn sample_config() {
    let test_cfg = r#"
runner: python
install:
  artifacts: "https://raw.githubusercontent.com/Tectone23/MobileRuntime/main/config.json"
  cmd_nix:
    - "mv config.json ~/.tre-config.json"
    - "./src/main.py --headless-init"
actions:
  run_api: "--cog_path={cog_path}"
  run_interactive: "--cog_path={cog_path} --interactive"
  version: "--version"
      "#;

    let yaml: Result<ComponentConfig, serde_yaml::Error> = serde_yaml::from_str(test_cfg);

    match yaml {
        Ok(yaml) => {
            println!("{}", serde_yaml::to_string(&yaml).unwrap());
            println!("{:?}", yaml);
        }
        Err(err) => throw(err.to_string().as_str()),
    }
}
