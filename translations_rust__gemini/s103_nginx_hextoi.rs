use std::io::{self, Read};

fn ngx_hextoi(line: &[u8]) -> i64 {
    if line.is_empty() {
        return -1;
    }

    let cutoff = i64::MAX / 16;
    let mut value: i64 = 0;

    for &ch in line {
        if value > cutoff {
            return -1;
        }

        if ch >= b'0' && ch <= b'9' {
            value = value.wrapping_mul(16).wrapping_add((ch - b'0') as i64);
            continue;
        }

        let c = ch | 0x20;

        if c >= b'a' && c <= b'f' {
            value = value.wrapping_mul(16).wrapping_add((c - b'a' + 10) as i64);
            continue;
        }

        return -1;
    }

    value
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buf = [0u8; 127];
    let mut len = 0;

    while len < 127 {
        let mut b = [0u8; 1];
        match stdin.read(&mut b) {
            Ok(1) => {
                buf[len] = b[0];
                len += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let slice = &buf[..len];
    let end = slice
        .iter()
        .position(|&b| b == b'\r' || b == b'\n' || b == 0)
        .unwrap_or(len);
    let line = &slice[..end];

    let r = ngx_hextoi(line);
    println!("{}", r);
}
