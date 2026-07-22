use std::io::{self, Read};

fn make_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut rem = i;
        for _ in 0..8 {
            if rem & 1 != 0 {
                rem = (rem >> 1) ^ 0xedb88320;
            } else {
                rem >>= 1;
            }
        }
        table[i as usize] = rem;
    }
    table
}

fn rc_crc32(crc: u32, buf: &[u8]) -> u32 {
    let table = make_table();
    let mut crc = !crc;
    for &octet in buf {
        crc = (crc >> 8) ^ table[((crc & 0xff) ^ octet as u32) as usize];
    }
    !crc
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");

    println!("{:X}", rc_crc32(0, line.as_bytes()));
}
