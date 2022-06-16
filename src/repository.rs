use std::{env, error::Error, path::Path};

use git2;

mod https;
mod ssh;

pub struct Repository {
    _repo: git2::Repository,
}

impl Repository {
    pub fn clone(git_url: &str, path: &Path) -> Result<Repository, Box<dyn Error>> {
        let mut callbacks = git2::RemoteCallbacks::new();

        callbacks.credentials(|_url, username, allowed_types| {
            let cred = git2::Cred::ssh_key(
                username.unwrap(),
                None,
                Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            );

            cred
        });

        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        let repo = builder.clone(git_url, path)?;

        Ok(Repository { _repo: repo })
    }
}
