use std::io::{self, Read};
use std::os::raw::c_ulong;

fn main() {
    let mut stdin = io::stdin().lock();
    let mut byte = [0u8; 1];

    let mut found_non_ws = false;
    let mut first_byte = 0u8;
    while let Ok(1) = stdin.read(&mut byte) {
        let b = byte[0];
        if !b.is_ascii_whitespace() {
            found_non_ws = true;
            first_byte = b;
            break;
        }
    }

    if !found_non_ws {
        return;
    }

    let mut input_bytes = Vec::with_capacity(63);
    input_bytes.push(first_byte);

    while input_bytes.len() < 63 {
        if let Ok(1) = stdin.read(&mut byte) {
            let b = byte[0];
            if b.is_ascii_whitespace() {
                break;
            }
            input_bytes.push(b);
        } else {
            break;
        }
    }

    let mut v: c_ulong = 0;
    for &b in &input_bytes {
        if b < b'0' || b > b'9' {
            break;
        }
        v = v.wrapping_mul(10).wrapping_add((b - b'0') as c_ulong);
    }

    println!("{}", v);
}
