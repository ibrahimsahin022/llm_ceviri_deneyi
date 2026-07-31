use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4096];
    let mut len = 0;
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    while len < 4095 {
        let mut byte = [0u8; 1];
        match handle.read(&mut byte) {
            Ok(1) => {
                buf[len] = byte[0];
                len += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let mut h: u32 = 0;
    for &b in &buf[..len] {
        if b == b'\n' || b == b'\r' {
            break;
        }
        h = (h << 4).wrapping_add(b as u32);
        let g = h & 0xF000_0000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= !g;
    }

    println!("{}", h);
}
