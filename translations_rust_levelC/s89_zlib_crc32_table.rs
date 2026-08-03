use std::io::{self, BufRead};

const POLY: u32 = 0xedb88320;

fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for n in 0..256u32 {
        let mut c = n;
        for _ in 0..8 {
            c = if c & 1 != 0 { POLY ^ (c >> 1) } else { c >> 1 };
        }
        table[n as usize] = c;
    }
    table
}

fn crc32_impl(crc_in: u32, buf: &[u8], table: &[u32; 256]) -> u32 {
    let mut crc = crc_in ^ 0xffffffff;
    for &b in buf {
        crc = (crc >> 8) ^ table[((crc ^ b as u32) & 0xff) as usize];
    }
    crc ^ 0xffffffff
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let table = make_crc_table();
    let result = crc32_impl(0, line.as_bytes(), &table);
    println!("{}", result);
}
