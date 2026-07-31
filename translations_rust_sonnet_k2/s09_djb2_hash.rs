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

    let mut hash: u32 = 5381;
    for &b in line {
        if b == b'\n' || b == b'\r' {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(b as u32);
    }

    println!("{}", hash);
}
