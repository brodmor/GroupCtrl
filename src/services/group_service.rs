use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
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
    history: Arc<RwLock<VecDeque<String>>>,
}

impl GroupService {
    pub fn new(config_reader: ConfigReader) -> Self {
        let history = Arc::new(RwLock::new(VecDeque::new()));
        Self::spawn_history_writer(history.clone());
        Self {
            config_reader,
            history,
        }
    }

    fn spawn_history_writer(history: Arc<RwLock<VecDeque<String>>>) {
        let rx = System::observe_app_activations();
        thread::spawn(move || {
            for app_id in rx {
                let mut history = history.write().unwrap();
                history.retain(|aid| aid != &app_id);
                history.push_front(app_id);
                history.truncate(MAX_HISTORY);
            }
        });
    }

    pub async fn open(&self, group_id: Uuid) {
        let group = self.config_reader.read().group(group_id).unwrap().clone();
        let apps = group.apps();
        if let Some(app) = self
            .next_app(apps)
            .or_else(|| group.target.clone())
            .or_else(|| self.find_in_history(apps)) // most recent
            .or_else(|| apps.first().cloned())
        {
            Self::open_app(&app).await;
        }
    }

    fn next_app(&self, apps: &[App]) -> Option<App> {
        let current = System::current_app().ok()??;
        let pos = apps.iter().position(|app| app == &current)?;
        let next_pos = (pos + 1) % apps.len();
        Some(apps[next_pos].clone())
    }

    fn find_in_history(&self, apps: &[App]) -> Option<App> {
        self.history
            .read()
            .unwrap()
            .iter()
            .find_map(|id| apps.iter().find(|a| a.id() == *id))
            .cloned()
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
