use std::io::{self, BufRead};

fn memrchr(s: &[u8], c: u8) -> Option<usize> {
    let mut n = s.len();
    while n > 0 {
        n -= 1;
        if s[n] == c {
            return Some(n);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let text = lines.next().unwrap().unwrap();
    let ch_line = lines.next().unwrap().unwrap();
    if ch_line.is_empty() {
        return;
    }
    let c = ch_line.as_bytes()[0];
    match memrchr(text.as_bytes(), c) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}
