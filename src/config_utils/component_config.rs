use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentConfig {
    pub runner: Runners,
    pub install: Install,
    pub actions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Install {
    pub artifacts: Vec<String>,
    pub cmd_nix: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Runners {
    #[serde(rename = "python")]
    Python
}
