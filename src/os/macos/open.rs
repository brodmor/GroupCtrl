use super::app::App;
use crate::os::prelude::Openable;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        info!("Opening app {self}");
        let workspace = NSWorkspace::sharedWorkspace();
        let bundle_id = NSString::from_str(self.bundle_id.as_str());
        let Some(app_url) = workspace.URLForApplicationWithBundleIdentifier(&bundle_id) else {
            bail!("Could not find app with bundle id '{bundle_id}'");
        };
        // TODO use openApplicationAtUrl (requires async)
        if !workspace.openURL(&app_url) {
            let default_path = NSString::from_str("<no path found>");
            let app_path = app_url.path().unwrap_or(default_path).to_string();
            bail!("System refused to open app at path '{app_path}'");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_current_app() -> App {
        let workspace = NSWorkspace::sharedWorkspace();
        let focused_app = workspace
            .frontmostApplication()
            .expect("Could not find current app");
        let bundle_id = focused_app
            .bundleIdentifier()
            .expect("Could not find bundle id for current app")
            .to_string();
        App { bundle_id }
    }

    #[test]
    fn open_finder() {
        let initial_app = get_current_app();
        // This only means the command was received, but should be fine
        assert!(App::new("com.apple.finder").open().is_ok());
        initial_app.open().unwrap(); // restore focus
    }

    #[test]
    fn open_fake_app() {
        let fake_app = App::new("test.fake.app");
        assert!(fake_app.open().is_err());
    }
}
