#![warn(clippy::clone_on_ref_ptr)]

use std::sync::Arc;

use color_eyre::eyre::{eyre, Result};
use git2::Repository;
use tracing::info;
use tracing_chrome::ChromeLayerBuilder;
use tracing_chrome::FlushGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use crate::{config::Config, working_directory::WorkingDirectory};

mod config;
mod integration_state;
mod working_directory;

fn install_tracing() -> FlushGuard {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let (chrome_layer, guard) = ChromeLayerBuilder::new()
        .include_args(true)
        .file("trace.json")
        .build();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .with(chrome_layer)
        .init();

    guard
}

fn main() -> Result<()> {
    let _guard = install_tracing();
    color_eyre::install()?;

    let config = Config::from_json(
        std::env::args()
            .nth(1)
            .ok_or_else(|| eyre!("Missing path to config"))?,
    )?;
    info!("Read configuration: {:#?}", config);
    let config = Arc::new(config);

    let wd = WorkingDirectory::new(Arc::clone(&config))?;
    show_repo_info("Parent", wd.parent())?;
    show_repo_info("Child in parent", &wd.child_in_parent()?.open()?)?;
    show_repo_info("Child", wd.child())?;

    Ok(())
}

fn show_repo_info(name: &str, repo: &Repository) -> Result<()> {
    info!(
        "{}: {:?} ({})",
        name,
        repo.state(),
        repo.workdir()
            .ok_or_else(|| eyre!(format!("{} has now workdir", name)))?
            .display()
    );
    Ok(())
}
