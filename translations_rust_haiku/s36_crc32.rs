use std::io::{self, BufRead};
use std::sync::OnceLock;

fn get_table() -> &'static [u32; 256] {
    static TABLE: OnceLock<[u32; 256]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [0u32; 256];
        for i in 0..256 {
            let mut rem = i as u32;
            for _ in 0..8 {
                if rem & 1 != 0 {
                    rem >>= 1;
                    rem ^= 0xedb88320;
                } else {
                    rem >>= 1;
                }
            }
            t[i] = rem;
        }
        t
    })
}

fn rc_crc32(mut crc: u32, buf: &[u8]) -> u32 {
    let table = get_table();
    crc = !crc;
    for &octet in buf {
        crc = (crc >> 8) ^ table[((crc & 0xff) ^ (octet as u32)) as usize];
    }
    !crc
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut line)) = lines.next() {
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }
        let crc = rc_crc32(0, line.as_bytes());
        println!("{:X}", crc);
    }
}
