#![warn(clippy::clone_on_ref_ptr)]

use std::sync::Arc;

use crate::{config::Config, working_directory::WorkingDirectory};

mod config;
mod integration_state;
mod working_directory;

fn main() {
    let config_file = std::fs::File::open(std::env::args().nth(1).unwrap()).unwrap();
    let config: Config = serde_json::from_reader(config_file).unwrap();
    let config = Arc::new(config);
    println!("{:#?}", config);

    let wd = WorkingDirectory::new(Arc::clone(&config)).unwrap();
    println!("{:?}", wd.parent().state());
}
