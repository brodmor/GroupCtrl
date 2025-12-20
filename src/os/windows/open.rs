use super::app::App;
use crate::os::prelude::Openable;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
