use std::fmt::{Display, Formatter};

use global_hotkey::hotkey::{Code, HotKey as GlobalHotkey, Modifiers};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::os::{KeyboardBehavior, System};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hotkey(pub GlobalHotkey); // We need the GlobalHotkey for registration

impl Serialize for Hotkey {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Hotkey {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl Hotkey {
    pub fn new(mods: Modifiers, key: Code) -> Self {
        Self(GlobalHotkey::new(Some(mods), key))
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (modifier, text) in System::modifier_format() {
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
