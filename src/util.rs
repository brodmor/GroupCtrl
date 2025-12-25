use global_hotkey::hotkey::Code;

pub fn capitalize(s: &str) -> String {
    s.chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &s[c.len_utf8()..])
        .unwrap_or_default()
}

pub fn is_modifier(code: &Code) -> bool {
    let code_str = code.to_string();
    code_str.contains("Control")
        || code_str.contains("Meta")
        || code_str.contains("Alt")
        || code_str.contains("Shift")
}
