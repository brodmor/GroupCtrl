use crate::os::prelude::AppTrait;
use crate::util::capitalize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    bundle_id: String,
}

impl AppTrait for App {
    fn new(bundle_id: &str) -> Self {
        Self {
            bundle_id: bundle_id.to_string(),
        }
    }

    fn id(&self) -> &str {
        self.bundle_id.as_str()
    }

    fn display(&self) -> String {
        let name = self
            .bundle_id
            .split(".")
            .last()
            .unwrap_or(self.bundle_id.as_str());
        capitalize(name)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
