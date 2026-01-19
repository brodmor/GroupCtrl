use objc2_app_kit::NSWorkspace;

use crate::os::{App, AppQuery, System};

impl AppQuery for System {
    fn current_app() -> anyhow::Result<Option<App>> {
        Ok(NSWorkspace::sharedWorkspace()
            .frontmostApplication()
            .and_then(|app| app.bundleIdentifier())
            .map(|bid| App {
                bundle_id: bid.to_string(),
            }))
    }
}
