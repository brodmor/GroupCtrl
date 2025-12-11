pub fn capitalize(s: &str) -> String {
    s.chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &s[c.len_utf8()..])
        .unwrap_or_default()
}
