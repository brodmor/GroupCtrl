use std::fmt::{Display, Formatter};

use crate::os::{App, Openable};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Action {
    OpenApp(App),
    #[cfg(test)]
    Mock(&'static str),
}

impl Action {
    pub fn execute(&self) -> anyhow::Result<()> {
        match self {
            Action::OpenApp(app) => app.open()?,
            #[cfg(test)]
            Action::Mock(_) => {}
        };
        Ok(())
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Action::OpenApp(app) => format!("Open {app}"),
            #[cfg(test)]
            Action::Mock(str) => format!("Mock {str}"),
        };
        write!(f, "{msg}")
    }
}
