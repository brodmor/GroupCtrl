use crate::os::prelude::FormatTrait;
use global_hotkey::hotkey::Modifiers;

pub struct Format;

impl FormatTrait for Format {
    fn modifiers() -> [(Modifiers, &'static str); 4] {
        [
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SUPER, "Win+"),
            (Modifiers::ALT, "Alt+"),
            (Modifiers::SHIFT, "Shift+"),
        ]
    }
}
