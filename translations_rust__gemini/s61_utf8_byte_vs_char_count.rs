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

    let mut bytes = 0;
    let mut chars = 0;
    for &c in &buf[..len] {
        if c == b'\0' || c == b'\n' || c == b'\r' {
            break;
        }
        bytes += 1;
        if (c & 0xC0) != 0x80 {
            chars += 1;
        }
    }

    println!("{} {}", bytes, chars);
}
