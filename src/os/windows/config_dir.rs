use std::path::PathBuf;

use crate::os::{ConfigDir, System};

impl ConfigDir for System {
    fn config_dir() -> PathBuf {
        dirs::config_dir().expect("Could not find config directory")
    }
}
