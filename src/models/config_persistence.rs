use std::path::PathBuf;

use super::Config;

const CONFIG_FILE_NAME: &str = "config.yaml";

impl Config {
    fn path() -> PathBuf {
        crate::os::config_dir().join(CONFIG_FILE_NAME)
    }

    pub fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(Self::path())?;
        Ok(serde_yaml::from_str(&content)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = Self::path().parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_yaml::to_string(self)?;
        std::fs::write(Self::path(), content)?;
        Ok(())
    }
}
