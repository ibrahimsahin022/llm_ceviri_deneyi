use std::io::{self, BufRead};

fn ngx_atoi(line: &[u8]) -> i64 {
    if line.is_empty() {
        return -1;
    }

    let cutoff = i64::MAX / 10;
    let cutlim = i64::MAX % 10;
    let mut value: i64 = 0;

    for &byte in line {
        if byte < b'0' || byte > b'9' {
            return -1;
        }

        let digit = (byte - b'0') as i64;
        if value >= cutoff && (value > cutoff || digit > cutlim) {
            return -1;
        }

        value = value * 10 + digit;
    }

    value
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_ok() && !line.is_empty() {
        let trimmed = line.trim_end_matches(&['\r', '\n'][..]);
        let r = ngx_atoi(trimmed.as_bytes());
        println!("{}", r);
    }
}
