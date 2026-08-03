use std::io::{self, BufRead};

fn memrchr(data: &[u8], c: u8) -> Option<usize> {
    let mut n = data.len();
    while n > 0 {
        n -= 1;
        if data[n] == c {
            return Some(n);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut text = String::new();
    let mut ch = String::new();

    if handle.read_line(&mut text).is_ok() && handle.read_line(&mut ch).is_ok() {
        text = text.trim_end_matches(&['\r', '\n'][..]).to_string();
        ch = ch.trim_end_matches(&['\r', '\n'][..]).to_string();

        if !ch.is_empty() {
            let c_byte = ch.as_bytes()[0];
            match memrchr(text.as_bytes(), c_byte) {
                Some(idx) => println!("{}", idx),
                None => println!("-1"),
            }
        }
    }
}
