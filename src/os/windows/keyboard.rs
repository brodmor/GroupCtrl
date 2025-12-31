use global_hotkey::hotkey::Modifiers;

use crate::os::KeyboardBehavior;

pub struct Keyboard;

impl KeyboardBehavior for Keyboard {
    fn modifier_format() -> [(Modifiers, &'static str); 4] {
        [
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SUPER, "Win+"),
            (Modifiers::ALT, "Alt+"),
            (Modifiers::SHIFT, "Shift+"),
        ]
    }

    fn is_multi_select(modifiers: Modifiers) -> bool {
        modifiers.ctrl()
    }
}
