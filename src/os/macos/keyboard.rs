use global_hotkey::hotkey::Modifiers;

use crate::os::{KeyboardBehavior, ModifierFormat, System};

impl KeyboardBehavior for System {
    fn serde_modifier_format() -> ModifierFormat {
        [
            (Modifiers::META, "Cmd"),
            (Modifiers::ALT, "Opt"),
            (Modifiers::CONTROL, "Ctrl"),
            (Modifiers::SHIFT, "Shift"),
        ]
    }

    fn gui_modifier_format() -> ModifierFormat {
        [
            (Modifiers::CONTROL, "⌃"),
            (Modifiers::ALT, "⌥"),
            (Modifiers::SHIFT, "⇧"),
            (Modifiers::META, "⌘"),
        ]
    }

    fn key_sep() -> &'static str {
        ""
    }

    fn is_multi_select(modifiers: Modifiers) -> bool {
        modifiers.meta()
    }
}
