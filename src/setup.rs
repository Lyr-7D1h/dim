use std::{
    env,
    error::Error,
    fs::{create_dir_all, read_dir},
    path::{Path, PathBuf},
};

use dirs::config_dir;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use ssh2::Session;

fn https_authentication() -> Cred {
    todo!()
}

pub fn setup(git_url: &str) -> Result<(), Box<dyn Error>> {
    let repository_path = config_dir()
        .ok_or("Could not find config directory")?
        .join("dimport/.repository");

    if !repository_path.exists() {
        create_dir_all(&repository_path)?;
    }

    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_url, username, allowed_types| {
        let cred = Cred::ssh_key(
            username.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        );

        cred
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    builder.clone(git_url, &repository_path)?;

    Ok(())
}
