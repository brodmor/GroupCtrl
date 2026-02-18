use global_hotkey::hotkey::{Code, Modifiers};

use super::Hotkey;
use crate::os::{KeyboardBehavior, ModifierFormat, System};

const SERDE_SEP: &str = "+";
const KEY_PREFIXES: [&str; 4] = ["Key", "Digit", "Arrow", ""];

pub(super) fn show_hotkey_parts(hotkey: &Hotkey) -> Vec<String> {
    let mut parts = mods_to_string_vec(hotkey.mods, System::gui_modifier_format());
    let key_show = System::show_key(hotkey.key).unwrap_or_else(|| show_key_common(hotkey.key));
    parts.push(key_show);
    parts
}

impl From<Hotkey> for String {
    fn from(hotkey: Hotkey) -> Self {
        let mut parts = mods_to_string_vec(hotkey.mods, System::serde_modifier_format());
        parts.push(key_to_string(hotkey.key));
        parts.join(SERDE_SEP)
    }
}

impl From<String> for Hotkey {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split(SERDE_SEP).collect();
        let (mod_parts, key_part) = parts.split_at(parts.len() - 1);
        let mods = parse_mods(mod_parts, System::serde_modifier_format());
        let key = parse_key(key_part[0]);
        Hotkey::new(mods, key)
    }
}

fn mods_to_string_vec(mods: Modifiers, modifier_format: ModifierFormat) -> Vec<String> {
    modifier_format
        .iter()
        .filter(|(m, _)| mods.contains(*m))
        .map(|(_, text)| text.to_string())
        .collect()
}

fn key_to_string(key: Code) -> String {
    let key_str = key.to_string();
    KEY_PREFIXES
        .iter()
        .find_map(|prefix| key_str.strip_prefix(prefix))
        .unwrap() // safe since str.strip_prefix("") is no-op
        .to_string()
}

fn show_key_common(key: Code) -> String {
    match key {
        Code::Backslash => "\\".to_string(),
        Code::Slash => "/".to_string(),
        Code::Semicolon => ";".to_string(),
        Code::Quote => "'".to_string(),
        Code::Comma => ",".to_string(),
        Code::Period => ".".to_string(),
        Code::Backquote => "`".to_string(),
        Code::BracketLeft => "[".to_string(),
        Code::BracketRight => "]".to_string(),
        Code::Minus => "-".to_string(),
        Code::Equal => "=".to_string(),
        _ => key_to_string(key),
    }
}

fn parse_part(part: &str, modifier_format: ModifierFormat) -> Modifiers {
    modifier_format
        .iter()
        .find(|(_, text)| *text == part)
        .map(|(m, _)| *m)
        .unwrap_or_else(|| panic!("unknown modifier: {part}"))
}

fn parse_mods(parts: &[&str], modifier_format: ModifierFormat) -> Modifiers {
    parts
        .iter()
        .map(|part| parse_part(part, modifier_format))
        .fold(Modifiers::empty(), |acc, m| acc | m)
}

fn parse_key(string: &str) -> Code {
    KEY_PREFIXES
        .iter()
        .find_map(|prefix| format!("{prefix}{string}").parse::<Code>().ok())
        .unwrap_or_else(|| panic!("unknown key: {string}"))
}
