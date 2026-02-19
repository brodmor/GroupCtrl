use std::path::PathBuf;

use crate::os::{ConfigDir, System};

const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub fn config_dir() -> PathBuf {
    System::config_dir().join(APP_NAME.to_lowercase())
}

pub fn icons_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("could not determine data local directory")
        .join(APP_NAME)
        .join("icons")
}
