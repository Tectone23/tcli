use serde::{Deserialize, Serialize};

pub fn init_hooks(name: &str) -> String {
    let file = InitHooksFile::new(name);
    return serde_yaml::to_string::<InitHooksFile>(&file).unwrap();
}

pub fn init_cogs(name: &str) -> String {
    let file = InitCogsFile::new(name);
    return serde_yaml::to_string::<InitCogsFile>(&file).unwrap();
}

#[derive(Deserialize, Serialize)]
struct InitHooksFile {
    hook: Hooks,
    config: Config,
}

impl InitHooksFile {
    pub fn new(name: &str) -> Self {
        Self {
            hook: Hooks {
                namespace: name.to_string(),
                source_path: "src/".to_string(),
                depends: vec![],
                actions: None,
            },
            config: Config {
                example_api_url: "".to_string(),
            },
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
struct Hooks {
    namespace: String,
    source_path: String,
    depends: Vec<String>,
    actions: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Default)]
struct Config {
    example_api_url: String,
}

#[derive(Deserialize, Serialize)]
struct InitCogsFile {
    name: String,
    namespace: String,
}

impl InitCogsFile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: name.to_string(),
        }
    }
}

// the new cog format
#[derive(Deserialize, Serialize, Default)]
struct NewConfig {
    name: String,
    version: String,
    license: String,
    repo: String,
    issues: String,
    hook: Hooks,
    config: Config,
}
