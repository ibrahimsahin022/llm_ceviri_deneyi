use std::io::{self, BufRead};

fn ngx_hextoi(line: &[u8]) -> i32 {
    if line.is_empty() {
        return -1;
    }

    let cutoff = i32::MAX / 16;
    let mut value: i32 = 0;

    for &ch in line {
        if value > cutoff {
            return -1;
        }

        if ch.is_ascii_digit() {
            value = value * 16 + (ch - b'0') as i32;
            continue;
        }

        let c = ch | 0x20;
        if (b'a'..=b'f').contains(&c) {
            value = value * 16 + (c - b'a' + 10) as i32;
            continue;
        }

        return -1;
    }

    value
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", ngx_hextoi(line.as_bytes()));
}
