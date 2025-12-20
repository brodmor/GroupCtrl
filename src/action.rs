use crate::os::App;
use crate::os::prelude::*;
use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    OpenApp(App),
}

impl Action {
    pub fn execute(&self) -> Result<()> {
        match self {
            Action::OpenApp(app) => app.open()?,
        };
        Ok(())
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
