use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::models::Identifiable;
use crate::util::capitalize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct App {
    pub(super) bundle_id: String,
}

impl Identifiable<String> for App {
    fn id(&self) -> String {
        self.bundle_id.clone()
    }
}

impl App {
    pub fn display(&self) -> String {
        let name = self.bundle_id.split(".").last().unwrap_or(&self.bundle_id);
        capitalize(name)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
