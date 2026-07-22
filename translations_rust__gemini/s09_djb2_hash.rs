use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    while buf.len() < 4095 {
        if let Ok(1) = handle.read(&mut byte) {
            buf.push(byte[0]);
            if byte[0] == b'\n' {
                break;
            }
        } else {
            break;
        }
    }

    if buf.is_empty() {
        return;
    }

    let mut hash: u32 = 5381;
    for &b in &buf {
        if b == b'\n' || b == b'\r' {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(b as u32);
    }

    println!("{}", hash);
}
