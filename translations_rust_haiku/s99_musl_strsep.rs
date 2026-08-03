use std::io::{self, BufRead};

fn my_strcspn(s: &[u8], sep: &[u8]) -> usize {
    for (i, &byte) in s.iter().enumerate() {
        if sep.iter().any(|&c| c == byte) {
            return i;
        }
    }
    s.len()
}

fn my_strsep(s: &mut &[u8], sep: &[u8]) -> Option<Vec<u8>> {
    if s.is_empty() {
        return None;
    }

    let len = my_strcspn(s, sep);
    let token = s[..len].to_vec();

    if len < s.len() {
        // Found a separator, skip it
        *s = &s[len + 1..];
    } else {
        // No separator found, end of string
        *s = &[];
    }

    Some(token)
}

fn main() {
    let stdin = io::stdin();
    let mut sep = String::new();
    let mut text = String::new();

    if stdin.read_line(&mut sep).is_err() {
        return;
    }
    if stdin.read_line(&mut text).is_err() {
        return;
    }

    sep = sep.trim_end().to_string();
    text = text.trim_end().to_string();

    let sep_bytes = sep.as_bytes();
    let mut p = text.as_bytes();

    while let Some(tok) = my_strsep(&mut p, sep_bytes) {
        if let Ok(s) = std::str::from_utf8(&tok) {
            println!("{}", s);
        }
    }
}
