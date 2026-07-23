use std::io::{self, Read};

fn rc_crc32(crc: u32, buf: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for i in 0..256 {
        let mut rem = i as u32;
        for _ in 0..8 {
            if rem & 1 != 0 {
                rem = (rem >> 1) ^ 0xedb88320;
            } else {
                rem >>= 1;
            }
        }
        table[i] = rem;
    }

    let mut crc = !crc;
    for &b in buf {
        let idx = ((crc & 0xff) ^ (b as u32)) as usize;
        crc = (crc >> 8) ^ table[idx];
    }
    !crc
}

fn main() {
    let mut buf = Vec::new();
    let mut handle = io::stdin().lock();
    let mut byte = [0u8; 1];

    while buf.len() < 1023 {
        match handle.read(&mut byte) {
            Ok(1) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if buf.is_empty() {
        return;
    }

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    println!("{:X}", rc_crc32(0, &buf));
}
