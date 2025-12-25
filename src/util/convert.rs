use std::str::FromStr;

use dioxus::prelude::*;
use global_hotkey::hotkey::{Code, Modifiers};

use crate::models::hotkey::Hotkey;

pub fn convert_hotkey(evt: &KeyboardEvent) -> Option<Hotkey> {
    let modifiers = convert_modifiers(evt.modifiers());
    let code = convert_key(evt.code())?;
    Some(Hotkey::new(modifiers, code))
}

fn convert_modifiers(mods: Modifiers) -> Modifiers {
    let mut result = Modifiers::empty();
    if mods.ctrl() {
        result |= Modifiers::CONTROL
    }
    if mods.meta() {
        result |= Modifiers::SUPER
    }
    if mods.alt() {
        result |= Modifiers::ALT
    }
    if mods.shift() {
        result |= Modifiers::SHIFT
    }
    result
}

fn convert_key(code: Code) -> Option<Code> {
    let code_str = code.to_string();
    if code_str.contains("Control")
        || code_str.contains("Meta")
        || code_str.contains("Alt")
        || code_str.contains("Shift")
    {
        return None;
    }
    Code::from_str(&code_str).ok()
}
