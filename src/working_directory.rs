use std::{path::Path, sync::Arc};

use color_eyre::eyre::{ContextCompat, Result};

use git2::{
    build::RepoBuilder, Cred, ErrorCode, FetchOptions, RemoteCallbacks, Repository, Submodule,
    SubmoduleUpdateOptions,
};
use tracing::info;

use crate::config::Config;

pub struct WorkingDirectory {
    config: Arc<Config>,
    parent: Repository,
    child: Repository,
}

impl WorkingDirectory {
    #[tracing::instrument(skip(config))]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let parent = Self::open_or_clone_repo(&config, &config.parent.name)?;
        let child = Self::open_or_clone_repo(&config, &config.child.name)?;

        Ok(Self {
            config: Arc::clone(&config),
            parent,
            child,
        })
    }

    pub fn parent(&self) -> &Repository {
        &self.parent
    }

    pub fn child(&self) -> &Repository {
        &self.child
    }

    pub fn child_in_parent(&self) -> Result<Submodule> {
        self.parent
            .submodules()?
            .into_iter()
            .find(|s| s.path() == self.config.parent.child_path)
            .wrap_err_with(|| {
                format!(
                    "Could not find submodule in parent repo at path `{}`",
                    self.config.parent.child_path.display()
                )
            })
    }

    fn default_fetch_options() -> FetchOptions<'static> {
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
                None,
            )
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);

        fetch_options
    }

    #[tracing::instrument(skip(config))]
    fn open_or_clone_repo(config: &Config, name: &str) -> Result<Repository> {
        let repo_path = config.working_directory.join(name);
        std::fs::create_dir_all(config.working_directory.join(name))?;

        let repo = match Repository::open(&repo_path) {
            Ok(repo) => repo,
            Err(e) if e.code() == ErrorCode::NotFound => Self::clone_repo(config, name, repo_path)?,
            Err(e) => return Err(e.into()),
        };

        Ok(repo)
    }

    #[tracing::instrument(skip(config))]
    fn clone_repo(
        config: &Config,
        name: &str,
        repo_path: std::path::PathBuf,
    ) -> Result<Repository> {
        info!("Cloning {}", name);
        let repo = RepoBuilder::new()
            .fetch_options(Self::default_fetch_options())
            .clone(
                &format!(
                    "git@{}:{}/{}.git",
                    config.github.domain, config.github.owner, name
                ),
                &repo_path,
            )?;
        for mut submodule in repo.submodules()? {
            submodule.update(
                true,
                Some(SubmoduleUpdateOptions::new().fetch(Self::default_fetch_options())),
            )?;
        }
        Ok(repo)
    }
}
