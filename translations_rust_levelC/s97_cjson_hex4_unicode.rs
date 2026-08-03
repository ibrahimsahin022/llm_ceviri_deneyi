use std::io::{self, BufRead};

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for i in 0..4 {
        let c = input[i];
        if c.is_ascii_digit() {
            h += (c - b'0') as u32;
        } else if (b'A'..=b'F').contains(&c) {
            h += 10 + (c - b'A') as u32;
        } else if (b'a'..=b'f').contains(&c) {
            h += 10 + (c - b'a') as u32;
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
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let r = parse_hex4(line.as_bytes());
    println!("{}", r);
}
