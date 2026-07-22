use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4096];
    let mut stdin = io::stdin().lock();
    let mut count = 0;

    while count < 4095 {
        let mut b = [0u8; 1];
        match stdin.read(&mut b) {
            Ok(1) => {
                buf[count] = b[0];
                count += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if count == 0 {
        return;
    }

    let mut hash: u32 = 2166136261;
    for &b in &buf[..count] {
        if b == b'\n' || b == b'\r' {
            break;
        }
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    println!("{}", hash);
}
