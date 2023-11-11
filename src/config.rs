use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub working_directory: PathBuf,
    pub webhook_listen_port: u16,
    pub github: Github,
    pub parent: Parent,
    pub child: Child,
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
