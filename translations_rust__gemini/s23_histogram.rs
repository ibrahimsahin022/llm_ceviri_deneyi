use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = Vec::new();
    if handle.read_until(b'\n', &mut buf).unwrap_or(0) == 0 {
        return;
    }
    if buf.len() > 4095 {
        buf.truncate(4095);
    }

    let mut freq = [0i32; 26];
    for &c in &buf {
        if c >= b'a' && c <= b'z' {
            freq[(c - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        if freq[i] > 0 {
            println!("{}:{}", (b'a' + i as u8) as char, freq[i]);
        }
    }
}
