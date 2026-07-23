use std::io::{self, Read};

fn luhn(cc: &[u8]) -> bool {
    let m = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut odd = true;
    let mut sum: i32 = 0;

    for &b in cc.iter().rev() {
        let digit = (b as i32) - (b'0' as i32);
        let val = if odd {
            digit
        } else if digit >= 0 && (digit as usize) < m.len() {
            m[digit as usize]
        } else {
            0
        };
        sum += val;
        odd = !odd;
    }

    sum % 10 == 0
}

fn main() {
    let mut buf = [0u8; 63];
    let mut len = 0;

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    while len < 63 {
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

    let mut line = &buf[..len];
    while let Some((&last, rest)) = line.split_last() {
        if last == b'\n' || last == b'\r' {
            line = rest;
        } else {
            break;
        }
    }

    if luhn(line) {
        println!("ok");
    } else {
        println!("not ok");
    }
}
