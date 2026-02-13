use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Hotkey;
use crate::models::traits::Identifiable;
use crate::os::App;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(skip, default = "Uuid::new_v4")]
    id: Uuid,
    pub name: String,
    pub hotkey: Option<Hotkey>,
    apps: Vec<App>,
}

impl Identifiable<Uuid> for Group {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            hotkey: None,
            apps: Vec::new(),
        }
    }

    pub fn apps(&self) -> &Vec<App> {
        &self.apps
    }

    pub(super) fn add_app(&mut self, app: App) {
        self.apps.push(app);
    }

    pub(super) fn remove_app(&mut self, app_id: String) {
        self.apps.retain(|a| a.id() != app_id)
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
