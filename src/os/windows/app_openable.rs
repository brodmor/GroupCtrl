use crate::os::{App, Openable};

mod open_app;
mod win32;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        open_app::open_app(&self.exe_path)
    }
}
