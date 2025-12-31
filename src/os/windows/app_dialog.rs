use crate::os::{App, AppSelection};

pub struct AppDialog;

impl AppSelection for AppDialog {
    async fn select_app() -> anyhow::Result<Option<App>> {
        todo!();
    }
}
