use std::io::{self, Read, Write};

fn is_space(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let len = input.len();
    let mut pos = 0usize;

    while pos < len && is_space(input[pos]) {
        pos += 1;
    }
    let start = pos;
    if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let digits_start = pos;
    while pos < len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if pos == digits_start {
        return;
    }
    let k_str = std::str::from_utf8(&input[start..pos]).unwrap();
    let k_val: i64 = match k_str.parse() {
        Ok(v) => v,
        Err(_) => return,
    };
    let k = ((k_val % 26) + 26) % 26;

    while pos < len && is_space(input[pos]) {
        pos += 1;
    }

    if pos >= len {
        return;
    }

    let line_start = pos;
    let mut line_end = pos;
    while line_end < len && input[line_end] != b'\n' {
        line_end += 1;
    }
    if line_end < len {
        line_end += 1;
    }
    let line = &input[line_start..line_end];

    let mut result: Vec<u8> = Vec::with_capacity(line.len() + 1);
    for &c in line {
        if c.is_ascii_lowercase() {
            result.push(b'a' + (((c - b'a') as i64 + k) % 26) as u8);
        } else if c.is_ascii_uppercase() {
            result.push(b'A' + (((c - b'A') as i64 + k) % 26) as u8);
        } else if c != b'\n' && c != b'\r' {
            result.push(c);
        }
    }
    result.push(b'\n');

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let _ = out.write_all(&result);
}
