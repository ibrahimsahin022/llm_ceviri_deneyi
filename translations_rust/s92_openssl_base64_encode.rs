use std::io::{self, BufRead};

const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn evp_encode_block(f: &[u8]) -> String {
    let mut out = Vec::new();
    let mut i = 0i32;
    let dlen = f.len() as i32;
    let mut fi = 0usize;
    let mut remaining = dlen;
    while remaining > 0 {
        if remaining >= 3 {
            let l = ((f[fi] as u32) << 16) | ((f[fi + 1] as u32) << 8) | (f[fi + 2] as u32);
            out.push(TABLE[((l >> 18) & 0x3f) as usize]);
            out.push(TABLE[((l >> 12) & 0x3f) as usize]);
            out.push(TABLE[((l >> 6) & 0x3f) as usize]);
            out.push(TABLE[(l & 0x3f) as usize]);
        } else {
            let mut l = (f[fi] as u32) << 16;
            if remaining == 2 {
                l |= (f[fi + 1] as u32) << 8;
            }
            out.push(TABLE[((l >> 18) & 0x3f) as usize]);
            out.push(TABLE[((l >> 12) & 0x3f) as usize]);
            if remaining == 1 {
                out.push(b'=');
            } else {
                out.push(TABLE[((l >> 6) & 0x3f) as usize]);
            }
            out.push(b'=');
        }
        i += 4;
        fi += 3;
        remaining -= 3;
    }
    let _ = i;
    String::from_utf8(out).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", evp_encode_block(line.as_bytes()));
}
