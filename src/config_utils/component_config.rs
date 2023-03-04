use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentConfig {
  install: Install,
  actions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
  artifacts: String,
  cmd_nix: Vec<String>,
}

pub fn sample_config() {
  let test_cfg = r#"
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

  let yaml: ComponentConfig = serde_yaml::from_str(test_cfg).unwrap();

  println!("{}", serde_yaml::to_string(&yaml).unwrap());
  println!("{:?}", yaml);
}
