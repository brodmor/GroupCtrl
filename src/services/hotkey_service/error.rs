use thiserror::Error;

use crate::models::Hotkey;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum HotkeyBindError {
    #[error("{hotkey} is already bound to {conflict}")]
    Conflict { hotkey: Hotkey, conflict: String },

    #[error("{hotkey} is not a valid hotkey")]
    Invalid { hotkey: Hotkey },

    #[error("{hotkey} could not be registered")]
    Unknown { hotkey: Hotkey },
}
