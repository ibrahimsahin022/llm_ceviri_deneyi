use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = [0u8; 4095];
    let mut count = 0;

    while count < 4095 {
        let mut b = [0u8; 1];
        if stdin.read_exact(&mut b).is_ok() {
            buf[count] = b[0];
            count += 1;
            if b[0] == b'\n' {
                break;
            }
        } else {
            break;
        }
    }

    if count == 0 {
        return;
    }

    let mut sum: i64 = 0;
    for &byte in &buf[..count] {
        if byte == 0 || byte == b'\n' || byte == b'\r' {
            break;
        }
        sum += (byte as i8) as i64;
    }

    println!("{}", sum);
}
