use std::io::{self, BufRead};
use std::collections::HashMap;

fn hash_key(key: &str) -> u64 {
    let mut h: u64 = 5381;
    for c in key.bytes() {
        h = ((h << 5) + h) + (c as u64);
    }
    h
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut table: HashMap<String, i64> = HashMap::new();

    for _ in 0..k {
        if let Some(Ok(mut line)) = lines.next() {
            while line.ends_with('\n') || line.ends_with('\r') {
                line.pop();
            }

            let parts: Vec<&str> = line.splitn(3, ' ').collect();

            if parts.is_empty() {
                continue;
            }

            let cmd = parts[0];
            if cmd == "INSERT" && parts.len() >= 2 {
                let key = parts[1].to_string();
                let value = if parts.len() >= 3 {
                    parts[2].parse::<i64>().unwrap_or(0)
                } else {
                    0
                };
                table.insert(key, value);
                println!("OK");
            } else if cmd == "GET" && parts.len() >= 2 {
                let key = parts[1];
                if let Some(&value) = table.get(key) {
                    println!("{}", value);
                } else {
                    println!("NOT_FOUND");
                }
            } else if cmd == "DEL" && parts.len() >= 2 {
                let key = parts[1];
                if table.remove(key).is_some() {
                    println!("OK");
                } else {
                    println!("NOT_FOUND");
                }
            }
        }
    }
}
