use std::io::{self, Read};

fn build_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut rem = i;
        for _ in 0..8 {
            if rem & 1 != 0 {
                rem >>= 1;
                rem ^= 0xedb88320;
            } else {
                rem >>= 1;
            }
        }
        table[i as usize] = rem;
    }
    table
}

fn rc_crc32(crc_in: u32, buf: &[u8], table: &[u32; 256]) -> u32 {
    let mut crc = !crc_in;
    for &octet in buf {
        crc = (crc >> 8) ^ table[((crc & 0xff) as u8 ^ octet) as usize];
    }
    !crc
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    if input.is_empty() {
        return;
    }
    let mut end = 0usize;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    let mut buf = input[..end].to_vec();

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    let table = build_table();
    let crc = rc_crc32(0, &buf, &table);

    println!("{:X}", crc);
}
