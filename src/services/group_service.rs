use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

use log::error;
use uuid::Uuid;

use crate::models::Identifiable;
use crate::os::{App, AppObserver, AppQuery, Openable, System};
use crate::services::ConfigReader;

const MAX_HISTORY: usize = 1024; // Prevent potential memory leak

#[derive(Clone)]
pub struct GroupService {
    config_reader: ConfigReader,
    history: Arc<Mutex<VecDeque<String>>>,
}

impl GroupService {
    pub fn new(config_reader: ConfigReader) -> Self {
        let history = Arc::new(Mutex::new(VecDeque::new()));
        Self::spawn_history_writer(history.clone());
        Self {
            config_reader,
            history,
        }
    }

    fn spawn_history_writer(history: Arc<Mutex<VecDeque<String>>>) {
        let rx = System::observe_app_activations();
        thread::spawn(move || {
            for app_id in rx {
                let mut history = history.lock().unwrap();
                history.retain(|aid| aid != &app_id);
                history.push_front(app_id);
                history.truncate(MAX_HISTORY);
            }
        });
    }

    pub async fn open(&self, group_id: Uuid) {
        let apps = self
            .config_reader
            .read()
            .group(group_id)
            .unwrap()
            .apps()
            .clone();
        if let Ok(Some(current)) = System::current_app()
            && let Some(pos) = apps.iter().position(|app| app == &current)
        {
            let next_pos = (pos + 1) % apps.len();
            Self::open_app(&apps[next_pos]).await;
        } else if let Some(app) = self.most_recent_app(&apps) {
            Self::open_app(&app).await;
        }
    }

    fn most_recent_app(&self, apps: &[App]) -> Option<App> {
        self.history
            .lock()
            .unwrap()
            .iter()
            .find_map(|id| apps.iter().find(|a| a.id() == *id))
            .cloned()
            .or_else(|| apps.first().cloned())
    }

    async fn open_app(app: &App) {
        let result = app.open().await;
        if let Err(error) = result {
            // This can fail because the app was uninstalled, etc
            error!(
                "Could not open app '{}' due to the following error: {}",
                app, error
            );
        }
    }
}
