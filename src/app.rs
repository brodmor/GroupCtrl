use crate::util::capitalize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    pub bundle_id: String,
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.bundle_id.split(".").last().unwrap();
        write!(f, "{}", capitalize(name))
    }
}

impl App {
    pub fn new(bundle_id: &str) -> App {
        Self {
            bundle_id: bundle_id.to_string(),
        }
    }
}
