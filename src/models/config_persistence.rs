use std::path::PathBuf;

use super::Config;
use crate::os::{ConfigDir, System};

const PERSISTENCE_DIR_NAME: &str = "GroupCtrl";
const CONFIG_FILE_NAME: &str = "config.yaml";

impl Config {
    fn path() -> PathBuf {
        System::config_dir()
            .join(PERSISTENCE_DIR_NAME)
            .join(CONFIG_FILE_NAME)
    }

    #[allow(unused)]
    pub fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(Self::path())?;
        Ok(serde_yaml::from_str(&content)?)
    }

    #[allow(unused)]
    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = Self::path().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_yaml::to_string(self)?;
        std::fs::write(Self::path(), content)?;
        Ok(())
    }
}
