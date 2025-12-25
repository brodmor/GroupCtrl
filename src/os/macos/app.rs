use std::fmt::{Display, Formatter};

use crate::util::capitalize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    pub bundle_id: String,
}

impl App {
    pub(super) fn new(bundle_id: &str) -> Self {
        Self {
            bundle_id: bundle_id.to_string(),
        }
    }

    fn display(&self) -> String {
        let name = self.bundle_id.split(".").last().unwrap_or(&self.bundle_id);
        capitalize(name)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
