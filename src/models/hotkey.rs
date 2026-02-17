use std::fmt::{Debug, Display, Formatter};

use global_hotkey::hotkey::{Code, HotKey as GlobalHotkey, Modifiers};
use serde::{Deserialize, Serialize};

use crate::models::hotkey_conversion::show_hotkey_parts;
use crate::os::{KeyboardBehavior, System};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "String", from = "String")]
pub struct Hotkey {
    pub(super) mods: Modifiers,
    pub(super) key: Code,
}

impl Hotkey {
    pub fn new(mods: Modifiers, key: Code) -> Hotkey {
        Self { mods, key }
    }

    pub fn global_hotkey(self) -> GlobalHotkey {
        GlobalHotkey::new(Some(self.mods), self.key)
    }

    pub fn show_parts(&self) -> Vec<String> {
        show_hotkey_parts(self)
    }
}

impl Display for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show_parts().join(System::key_sep()))
    }
}

impl Debug for Hotkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self)) // uses From impl, different from to_string
    }
}
