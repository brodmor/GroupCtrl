use log::error;
use uuid::Uuid;

use crate::os::{App, AppQuery, Openable, System};
use crate::services::ConfigService;

#[derive(Default)]
pub struct GroupService {}

impl GroupService {
    pub fn open(&self, config_service: &ConfigService, group_id: Uuid) {
        let apps = config_service.group(group_id).unwrap().apps();
        if let Ok(Some(current)) = System::current_app()
            && let Some(pos) = apps.iter().position(|app| app == &current)
        {
            let next_pos = (pos + 1) % apps.len();
            Self::open_app(&apps[next_pos]);
        } else if let Some(app) = apps.iter().next() {
            Self::open_app(app);
        }
    }

    fn open_app(app: &App) {
        let result = app.open();
        if let Err(error) = result {
            error!(
                "Could not open app '{}' due to the following error: {}",
                app, error
            );
        }
    }
}
