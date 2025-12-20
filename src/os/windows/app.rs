use crate::os::prelude::AppTrait;
use crate::util::capitalize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    exe_path: String,
}

impl AppTrait for App {
    fn id(&self) -> &str {
        self.exe_path.as_str()
    }

    fn new(exe_path: &str) -> Self {
        Self {
            exe_path: exe_path.to_string(),
        }
    }

    fn display(&self) -> String {
        let exe_name = self
            .exe_path
            .split("\\")
            .last()
            .unwrap_or(self.exe_path.as_str());
        let name = exe_name.strip_suffix(".exe").unwrap_or(exe_name);
        capitalize(name)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
