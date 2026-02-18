use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::app_metadata;
use crate::models::Identifiable;
use crate::os::AppMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub struct App {
    pub(super) bundle_id: String,
    pub(super) app_path: Option<String>,
    name: String,
    icon_path: Option<PathBuf>,
}

impl App {
    pub(super) fn new(
        bundle_id: String,
        app_path: Option<String>,
        name: String,
        icon_path: Option<PathBuf>,
    ) -> Self {
        Self {
            bundle_id,
            app_path,
            name,
            icon_path,
        }
    }
}

impl Identifiable<String> for App {
    fn id(&self) -> String {
        self.bundle_id.clone()
    }
}

impl AppMetadata for App {
    fn name(&self) -> &str {
        &self.name
    }

    fn icon_path(&self) -> Option<&Path> {
        self.icon_path.as_deref()
    }
}

impl From<App> for String {
    fn from(app: App) -> Self {
        app.bundle_id
    }
}

impl From<String> for App {
    fn from(bundle_id: String) -> Self {
        app_metadata::resolve(&bundle_id)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Hash for App {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bundle_id.hash(state);
    }
}

impl PartialEq for App {
    fn eq(&self, other: &Self) -> bool {
        self.bundle_id == other.bundle_id
    }
}

impl Eq for App {}
