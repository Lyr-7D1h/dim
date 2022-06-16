use std::{env, fs::create_dir_all, io, path::PathBuf};

use crate::dirs::config_dir;

/// Implements anything in the .config file
pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn init() -> io::Result<Config> {
        let path = config_dir().join("dim");

        create_dir_all(&path)?;

        Ok(Config { path })
    }

    pub fn new_repository() {}
}
