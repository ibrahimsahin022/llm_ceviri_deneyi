use std::io::{self, BufRead};
use std::num::Wrapping;

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

    let mut hash = Wrapping(0u32);
    for &byte in &buf {
        if byte == b'\n' || byte == b'\r' || byte == 0 {
            break;
        }
        let c = Wrapping(byte as u32);
        hash = c + (hash << 6) + (hash << 16) - hash;
    }

    println!("{}", hash.0);
}
