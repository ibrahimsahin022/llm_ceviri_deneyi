use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4096];
    let mut len = 0;
    let mut stdin = io::stdin().lock();

    while len < 4095 {
        let mut b = [0u8; 1];
        match stdin.read(&mut b) {
            Ok(1) => {
                buf[len] = b[0];
                len += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let mut alpha = 0;
    let mut digit = 0;

    for &b in &buf[..len] {
        if b == b'\0' || b == b'\n' || b == b'\r' {
            break;
        }
        if b.is_ascii_alphabetic() {
            alpha += 1;
        }
        if b.is_ascii_digit() {
            digit += 1;
        }
    }

    println!("{} {}", alpha, digit);
}
