use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4096];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut len = 0;

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

    let mut have = false;
    let mut mn: i8 = 0;
    let mut mx: i8 = 0;

    for &b in &buf[..len] {
        if b == 0 || b == b'\n' || b == b'\r' {
            break;
        }
        let c = b as i8;
        if !have {
            mn = c;
            mx = c;
            have = true;
        } else {
            if c < mn {
                mn = c;
            }
            if c > mx {
                mx = c;
            }
        }
    }

    println!("{} {}", mn, mx);
}
