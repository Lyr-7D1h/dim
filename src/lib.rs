use std::{error::Error, io};

use config::Config;

pub mod config;
pub mod repository;

mod dirs;

pub struct Dim {
    config: Config,
}

impl Dim {
    /// Initialize configuration files
    pub fn init() -> io::Result<Dim> {
        let config = Config::init()?;
        Ok(Dim { config })
    }

    pub fn sync(self) -> Result<(), Box<dyn Error>> {
        // TODO check if all files are complete
        // TODO see if there are diff with remote
        // TODO make remote branch for current changes
        todo!()
    }

    pub fn setup(self, git_url: String) -> Result<(), Box<dyn Error>> {
        // TODO link all files from .repository to actual repository
        todo!()
    }
}
