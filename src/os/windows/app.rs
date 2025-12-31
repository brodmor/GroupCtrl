use std::fmt::{Display, Formatter};

use crate::models::Identifiable;
use crate::util::capitalize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    pub(super) exe_path: String,
}

impl Identifiable<String> for App {
    fn id(&self) -> String {
        self.exe_path.clone()
    }
}

impl App {
    fn display(&self) -> String {
        let exe_name = self.exe_path.split("\\").last().unwrap_or(&self.exe_path);
        let name = exe_name.strip_suffix(".exe").unwrap_or(exe_name);
        capitalize(name)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
