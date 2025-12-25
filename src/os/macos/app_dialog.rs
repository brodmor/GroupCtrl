use std::fs::File;
use std::path::Path;

use anyhow::Context;

use crate::os::{App, AppSelection};

pub struct AppDialog;

impl AppSelection for AppDialog {
    async fn select_app() -> anyhow::Result<Option<App>> {
        let Some(app_path) = rfd::AsyncFileDialog::new()
            .add_filter("Applications", &["app"])
            .set_directory("/Applications")
            .pick_file()
            .await
        else {
            return Ok(None);
        };
        let bundle_id = get_bundle_id(app_path.path())?;
        Ok(Some(App { bundle_id }))
    }
}

fn get_bundle_id(app_path: &Path) -> anyhow::Result<String> {
    let plist_path = app_path.join("Contents/Info.plist");
    let file = File::open(&plist_path)?;
    let plist: plist::Value = plist::from_reader(file)?;
    plist
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleIdentifier"))
        .and_then(|value| value.as_string())
        .map(|s| s.to_string())
        .context("bundle identifier not found")
}
