use std::io::{self, BufRead};

fn ngx_atoi(line: &[u8]) -> i64 {
    if line.is_empty() {
        return -1;
    }

    let cutoff = i64::MAX / 10;
    let cutlim = i64::MAX % 10;
    let mut value: i64 = 0;

    for &c in line {
        if !c.is_ascii_digit() {
            return -1;
        }
        let d = (c - b'0') as i64;
        if value >= cutoff && (value > cutoff || d > cutlim) {
            return -1;
        }
        value = value * 10 + d;
    }

    value
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", ngx_atoi(line.as_bytes()));
}
