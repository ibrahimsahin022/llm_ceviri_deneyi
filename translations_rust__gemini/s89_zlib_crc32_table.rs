use std::io::{self, Read};

const POLY: u32 = 0xedb88320;

const fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut n = 0;
    while n < 256 {
        let mut c = n as u32;
        let mut k = 0;
        while k < 8 {
            c = if (c & 1) != 0 {
                POLY ^ (c >> 1)
            } else {
                c >> 1
            };
            k += 1;
        }
        table[n] = c;
        n += 1;
    }
    table
}

static CRC_TABLE: [u32; 256] = make_crc_table();

fn crc32_impl(mut crc: u32, buf: &[u8]) -> u32 {
    crc ^= 0xffffffff;
    let mut chunks = buf.chunks_exact(8);
    for chunk in &mut chunks {
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[0]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[1]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[2]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[3]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[4]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[5]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[6]) as usize];
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ chunk[7]) as usize];
    }
    for &b in chunks.remainder() {
        crc = (crc >> 8) ^ CRC_TABLE[((crc as u8) ^ b) as usize];
    }
    crc ^ 0xffffffff
}

fn main() {
    let mut buf = [0u8; 4095];
    let mut len = 0;

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    for byte in handle.bytes() {
        match byte {
            Ok(b) => {
                buf[len] = b;
                len += 1;
                if b == b'\n' || len == 4095 {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    if len == 0 {
        return;
    }

    let slice = &buf[..len];
    let trunc = slice.iter().position(|&b| b == b'\r' || b == b'\n').unwrap_or(len);
    let result = crc32_impl(0, &slice[..trunc]);
    println!("{}", result);
}
