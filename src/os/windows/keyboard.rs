use global_hotkey::hotkey::{Code, Modifiers};

use crate::os::{KeyboardBehavior, ModifierFormat, System};

impl KeyboardBehavior for System {
    fn serde_modifier_format() -> ModifierFormat {
        [
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::META, "Win"),
            (Modifiers::ALT, "Alt"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }

    fn gui_modifier_format() -> ModifierFormat {
        [
            (Modifiers::META, "Win"),
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::ALT, "Alt"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }

    fn key_sep() -> &'static str {
        "+"
    }

    fn is_multi_select(modifiers: Modifiers) -> bool {
        modifiers.ctrl()
    }

    fn show_key(_key: Code) -> Option<String> {
        None
    }
}
