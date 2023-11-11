use std::path::{Path, PathBuf};

use serde::Deserialize;

use color_eyre::eyre::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub working_directory: PathBuf,
    pub webhook_listen_port: u16,
    pub github: Github,
    pub parent: Parent,
    pub child: Child,
}

impl Config {
    pub fn from_json(path: impl AsRef<Path>) -> Result<Self> {
        let config_file = std::fs::File::open(path)?;
        let config: Config = serde_json::from_reader(config_file)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Github {
    pub domain: String,
    pub owner: String,
    pub user_access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Parent {
    pub name: String,
    pub dev_branch: String,
    pub child_path: PathBuf,
    pub current_integration_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Child {
    pub name: String,
    pub dev_branch: String,
    pub integration_branch: String,
    pub current_integration_branch: String,
}
