use crate::os::Format;
use crate::os::prelude::*;
use global_hotkey::hotkey::{Code, HotKey as GlobalHotkey, Modifiers};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hotkey(pub GlobalHotkey); // We need the GlobalHotkey for registration

impl Hotkey {
    pub fn new(mods: Modifiers, key: Code) -> Self {
        Self(GlobalHotkey::new(Some(mods), key))
    }

    pub fn id(&self) -> u32 {
        self.0.id
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (modifier, text) in Format::modifiers() {
            if self.0.mods.contains(modifier) {
                write!(f, "{}", text)?;
            }
        }
        let key_str = self.0.key.to_string();
        let key = ["Key", "Digit", "Arrow"]
            .iter()
            .find_map(|prefix| key_str.strip_prefix(prefix))
            .unwrap_or(&key_str);
        write!(f, "{}", key)
    }
}
