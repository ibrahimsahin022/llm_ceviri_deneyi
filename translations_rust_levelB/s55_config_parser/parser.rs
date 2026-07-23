use crate::config::{ConfigEntry, MAX_ENTRIES};

pub fn parse_config_lines(text: &str) -> Vec<ConfigEntry> {
    let mut entries = Vec::new();
    for raw_line in text.lines() {
        if entries.len() >= MAX_ENTRIES {
            break;
        }
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();
            entries.push(ConfigEntry { key, value });
        }
    }
    entries
}
