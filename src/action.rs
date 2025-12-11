use crate::app::App;
use crate::open::Open;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    OpenApp(App),
}

impl Action {
    pub fn execute(&self) {
        match self {
            Action::OpenApp(app) => app.open().unwrap(), // TODO error handling
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Action::OpenApp(app) => format!("Open {}", app),
        };
        write!(f, "{msg}")
    }
}
