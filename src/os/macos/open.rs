use anyhow::bail;
use log::info;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;

use super::app::App;
use crate::os::Openable;

impl Openable for App {
    fn open(&self) -> anyhow::Result<()> {
        info!("Opening app {self}");
        let workspace = NSWorkspace::sharedWorkspace();
        let bundle_id = NSString::from_str(&self.bundle_id);
        let Some(app_url) = workspace.URLForApplicationWithBundleIdentifier(&bundle_id) else {
            bail!("Could not find app with bundle id '{bundle_id}'");
        };
        // TODO use openApplicationAtUrl (requires async)
        if !workspace.openURL(&app_url) {
            bail!("syscall 'openURL' failed");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::{AppQuery, System};

    #[test]
    fn open_finder() {
        let initial_app = System::current_app();
        let app = App {
            bundle_id: "com.apple.finder".to_string(),
        };
        // This only means the command was received, but should be fine
        assert!(app.open().is_ok());
        if let Ok(Some(restore)) = initial_app {
            restore.open().unwrap();
        }
    }

    #[test]
    fn open_fake_app() {
        let fake_app = App {
            bundle_id: "com.test.fake".to_string(),
        };
        assert!(fake_app.open().is_err());
    }
}
