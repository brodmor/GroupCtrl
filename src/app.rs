use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct App {
    pub bundle_id: String,
}

fn capitalize(s: &str) -> String {
    s.chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &s[c.len_utf8()..])
        .unwrap_or_default()
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
