use std::io::{self, BufRead};

const POLY: u32 = 0xedb88320u32;

fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for n in 0..256 {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if (c & 1) != 0 {
                POLY ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        table[n] = c;
    }
    table
}

fn crc32_impl(crc: u32, buf: &[u8]) -> u32 {
    let crc_table = make_crc_table();

    let mut crc = crc ^ 0xffffffffu32;
    let mut pos = 0;
    let mut len = buf.len();

    while len >= 8 {
        len -= 8;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
    }

    while len > 0 {
        len -= 1;
        crc = (crc >> 8) ^ crc_table[(crc ^ buf[pos] as u32) as usize & 0xff];
        pos += 1;
    }

    crc ^ 0xffffffffu32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let result = crc32_impl(0, line.as_bytes());
        println!("{}", result);
    }
}
