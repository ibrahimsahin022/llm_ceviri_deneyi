use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    if input.is_empty() {
        return;
    }
    let mut end = 0usize;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    let line = &input[..end];

    let mut freq = [0i32; 26];
    for &b in line {
        if b >= b'a' && b <= b'z' {
            freq[(b - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        if freq[i] > 0 {
            println!("{}:{}", (b'a' + i as u8) as char, freq[i]);
        }
    }
}
