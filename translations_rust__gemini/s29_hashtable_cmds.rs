use std::collections::HashMap;
use std::io::{self, BufRead};

fn parse_long(s: &str) -> i64 {
    let s = s.trim_start();
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return 0;
    }
    let mut idx = 0;
    let mut sign = 1i64;
    if bytes[idx] == b'-' {
        sign = -1;
        idx += 1;
    } else if bytes[idx] == b'+' {
        idx += 1;
    }
    let mut val = 0i64;
    let mut found_digit = false;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        val = val.wrapping_mul(10).wrapping_add((bytes[idx] - b'0') as i64);
        found_digit = true;
        idx += 1;
    }
    if found_digit {
        val.wrapping_mul(sign)
    } else {
        0
    }
}

fn truncate_str(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        s
    } else {
        let mut end = max_bytes;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = match lines.next() {
        Some(Ok(l)) => l,
        _ => return,
    };

    let k = parse_long(&first_line);
    let mut map: HashMap<String, i64> = HashMap::new();

    for _ in 0..k {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => break,
        };

        let trimmed = line.trim_matches(|c| c == '\r' || c == '\n');

        let mut words = trimmed.split_whitespace();
        let cmd = match words.next() {
            Some(w) => w,
            None => continue,
        };
        let key = match words.next() {
            Some(w) => w,
            None => continue,
        };

        let cmd = truncate_str(cmd, 15);
        let key = truncate_str(key, 63);

        if cmd == "INSERT" {
            let value = match words.next() {
                Some(w) => parse_long(w),
                None => 0,
            };
            map.insert(key.to_string(), value);
            println!("OK");
        } else if cmd == "GET" {
            if let Some(&val) = map.get(key) {
                println!("{}", val);
            } else {
                println!("NOT_FOUND");
            }
        } else if cmd == "DEL" {
            if map.remove(key).is_some() {
                println!("OK");
            } else {
                println!("NOT_FOUND");
            }
        }
    }
}
