use std::io::{self, BufRead};

const MAX_ENTRIES: usize = 64;
const MAX_KEY_LEN: usize = 32;
const MAX_VAL_LEN: usize = 64;

#[derive(Clone)]
struct ConfigEntry {
    key: String,
    value: String,
}

fn trim(s: &mut String) {
    while !s.is_empty() && (s.ends_with('\n') || s.ends_with('\r') || s.ends_with(' ')) {
        s.pop();
    }

    let start = s.chars().take_while(|c| *c == ' ').count();
    if start > 0 {
        *s = s[start..].to_string();
    }
}

fn parse_config_lines(text: &str) -> Vec<ConfigEntry> {
    let mut entries = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    for line in lines {
        let mut line_str = line.to_string();
        trim(&mut line_str);

        if line_str.is_empty() || line_str.starts_with('#') {
            continue;
        }

        if let Some(eq_pos) = line_str.find('=') {
            let mut key = line_str[..eq_pos].to_string();
            let mut value = line_str[eq_pos + 1..].to_string();
            trim(&mut key);
            trim(&mut value);

            if key.len() >= MAX_KEY_LEN {
                key.truncate(MAX_KEY_LEN - 1);
            }
            if value.len() >= MAX_VAL_LEN {
                value.truncate(MAX_VAL_LEN - 1);
            }

            entries.push(ConfigEntry { key, value });

            if entries.len() >= MAX_ENTRIES {
                break;
            }
        }
    }
    entries
}

fn config_lookup(entries: &[ConfigEntry], key: &str) -> Option<String> {
    for entry in entries {
        if entry.key == key {
            return Some(entry.value.clone());
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut text = String::new();
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            text.push_str(&line);
            text.push('\n');
        }
    }

    let entries = parse_config_lines(&text);

    let m: usize = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    for _ in 0..m {
        if let Some(Ok(line)) = lines.next() {
            let key = line.trim();
            match config_lookup(&entries, key) {
                Some(val) => println!("{}", val),
                None => println!("NOT_FOUND"),
            }
        }
    }
}
