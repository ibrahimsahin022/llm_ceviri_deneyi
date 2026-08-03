use std::io::{self, BufRead};

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h = 0u32;

    for i in 0..4 {
        if i >= input.len() {
            return 0;
        }

        let byte = input[i];
        if byte >= b'0' && byte <= b'9' {
            h += (byte - b'0') as u32;
        } else if byte >= b'A' && byte <= b'F' {
            h += 10 + (byte - b'A') as u32;
        } else if byte >= b'a' && byte <= b'f' {
            h += 10 + (byte - b'a') as u32;
        } else {
            return 0;
        }

        if i < 3 {
            h <<= 4;
        }
    }

    h
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    if stdin.read_line(&mut line).is_err() {
        return;
    }

    let trimmed = line.trim_end();
    let result = parse_hex4(trimmed.as_bytes());
    println!("{}", result);
}
