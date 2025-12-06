use crate::app::App;
use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenAppError {
    #[error("Could not find app with bundle id '{bundle_id}'")]
    AppNotFound { bundle_id: String },
    #[error("System refused to open app at path '{app_path}'")]
    SystemRefused { app_path: String },
}

pub trait Open {
    fn open(&self) -> Result<(), OpenAppError>;
}

impl Open for App {
    fn open(&self) -> Result<(), OpenAppError> {
        let workspace = NSWorkspace::sharedWorkspace();
        let bundle_id = NSString::from_str(self.bundle_id.as_str());
        let Some(app_url) = workspace.URLForApplicationWithBundleIdentifier(&bundle_id) else {
            return Err(OpenAppError::AppNotFound {
                bundle_id: self.bundle_id.clone(),
            });
        };
        // TODO use openApplicationAtUrl (requires async)
        if !workspace.openURL(&app_url) {
            let default_path = NSString::from_str("<no path found>");
            let app_path = app_url.path().unwrap_or(default_path).to_string();
            return Err(OpenAppError::SystemRefused { app_path });
        }
        Ok(())
    }
}
