use std::io::{self, Read};

fn ngx_atoi(line: &[u8]) -> i64 {
    if line.is_empty() {
        return -1;
    }

    let cutoff = i64::MAX / 10;
    let cutlim = i64::MAX % 10;

    let mut value: i64 = 0;

    for &b in line {
        if b < b'0' || b > b'9' {
            return -1;
        }

        let digit = (b - b'0') as i64;

        if value >= cutoff && (value > cutoff || digit > cutlim) {
            return -1;
        }

        value = value * 10 + digit;
    }

    value
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = Vec::new();
    let mut byte = [0u8; 1];

    while line.len() < 63 {
        match handle.read(&mut byte) {
            Ok(1) => {
                line.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if line.is_empty() {
        return;
    }

    let len = line
        .iter()
        .position(|&c| c == b'\r' || c == b'\n')
        .unwrap_or(line.len());

    let r = ngx_atoi(&line[..len]);
    println!("{}", r);
}
