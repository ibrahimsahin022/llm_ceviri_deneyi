use std::io::{self, BufRead};

const DATA_BIN2ASCII: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn conv_bin2ascii(a: u32) -> u8 {
    DATA_BIN2ASCII[(a & 0x3f) as usize]
}

fn evp_encodeblock_int(f: &[u8]) -> String {
    let mut t = String::new();
    let mut i = f.len() as i32;
    let mut pos = 0;

    while i > 0 {
        if i >= 3 {
            let l = (((f[pos] as u32) << 16) |
                    ((f[pos + 1] as u32) << 8) |
                    (f[pos + 2] as u32));
            t.push(conv_bin2ascii(l >> 18) as char);
            t.push(conv_bin2ascii(l >> 12) as char);
            t.push(conv_bin2ascii(l >> 6) as char);
            t.push(conv_bin2ascii(l) as char);
        } else {
            let mut l = (f[pos] as u32) << 16;
            if i == 2 {
                l |= (f[pos + 1] as u32) << 8;
            }

            t.push(conv_bin2ascii(l >> 18) as char);
            t.push(conv_bin2ascii(l >> 12) as char);
            t.push(if i == 1 { '=' } else { conv_bin2ascii(l >> 6) as char });
            t.push('=');
        }

        i -= 3;
        pos += 3;
    }

    t
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let encoded = evp_encodeblock_int(line.as_bytes());
        println!("{}", encoded);
    }
}
