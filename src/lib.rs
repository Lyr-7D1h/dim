mod setup;
use std::path::PathBuf;

use dirs::{config_dir, home_dir};
pub use setup::setup;

mod sync;
pub use sync::sync;

mod credentials;

static HOME_DIR: PathBuf = home_dir().unwrap();
static CONFIG_DIR: PathBuf = config_dir().unwrap().join("dimport");
