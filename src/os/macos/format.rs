use crate::os::prelude::FormatTrait;
use global_hotkey::hotkey::Modifiers;

pub struct Format;

impl FormatTrait for Format {
    fn modifiers() -> [(Modifiers, &'static str); 4] {
        [
            (Modifiers::SUPER, "Cmd+"),
            (Modifiers::ALT, "Opt+"),
            (Modifiers::CONTROL, "Ctrl+"),
            (Modifiers::SHIFT, "Shift+"),
        ]
    }
}
