use std::io::{self, BufRead};

fn print_string_ptr(input: &[u8]) -> String {
    let mut out = String::new();
    out.push('"');
    for &b in input {
        match b {
            b'\"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            0x08 => out.push_str("\\b"),
            0x0c => out.push_str("\\f"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            _ => {
                if b < 32 {
                    out.push_str(&format!("\\u{:04x}", b));
                } else {
                    out.push(b as char);
                }
            }
        }
    }
    out.push('"');
    out
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", print_string_ptr(line.as_bytes()));
}
