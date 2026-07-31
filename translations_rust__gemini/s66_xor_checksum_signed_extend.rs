use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4095];
    let mut bytes_read = 0;
    let mut stdin = io::stdin().lock();

    while bytes_read < buf.len() {
        let mut b = [0u8; 1];
        match stdin.read(&mut b) {
            Ok(1) => {
                buf[bytes_read] = b[0];
                bytes_read += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if bytes_read == 0 {
        return;
    }

    let mut acc: u32 = 0;
    for &b in &buf[..bytes_read] {
        if b == b'\0' || b == b'\n' || b == b'\r' {
            break;
        }
        let c = b as i8;
        acc ^= c as i32 as u32;
    }

    println!("{}", acc);
}
