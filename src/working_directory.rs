use std::{path::Path, sync::Arc};

use anyhow::Context;
use git2::{
    build::RepoBuilder, Cred, ErrorCode, FetchOptions, RemoteCallbacks, Repository, Submodule,
    SubmoduleUpdateOptions,
};

use crate::config::Config;

pub struct WorkingDirectory {
    config: Arc<Config>,
    parent: Repository,
    child: Repository,
}

impl WorkingDirectory {
    pub fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let parent = Self::init_repo(&config.parent.name, &config)?;
        let child = Self::init_repo(&config.child.name, &config)?;

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

    pub fn child_in_parent(&self) -> anyhow::Result<Submodule> {
        self.parent
            .submodules()?
            .into_iter()
            .find(|s| dbg!(s.path()) == self.config.parent.child_path)
            .with_context(|| {
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

    fn init_repo(name: &str, config: &Config) -> anyhow::Result<Repository> {
        let repo_path = config.working_directory.join(name);
        std::fs::create_dir_all(config.working_directory.join(name))?;

        let repository = match Repository::open(&repo_path) {
            Ok(repo) => repo,
            Err(e) if e.code() == ErrorCode::NotFound => RepoBuilder::new()
                .fetch_options(Self::default_fetch_options())
                .clone(
                    &format!(
                        "git@{}:{}/{}.git",
                        config.github.domain, config.github.owner, name
                    ),
                    &repo_path,
                )
                .unwrap(),
            Err(e) => panic!("{:?}", e),
        };

        for mut submodule in repository.submodules()? {
            submodule.update(
                true,
                Some(SubmoduleUpdateOptions::new().fetch(Self::default_fetch_options())),
            )?;
        }

        Ok(repository)
    }
}
