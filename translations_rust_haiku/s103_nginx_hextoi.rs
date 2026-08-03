use std::io::{self, BufRead};

fn ngx_hextoi(line: &[u8]) -> i64 {
    const NGX_MAX_INT_T_VALUE: i64 = i64::MAX;

    if line.is_empty() {
        return -1;
    }

    let cutoff = NGX_MAX_INT_T_VALUE / 16;
    let mut value: i64 = 0;

    for &ch in line {
        if value > cutoff {
            return -1;
        }

        if ch >= b'0' && ch <= b'9' {
            value = value * 16 + (ch - b'0') as i64;
            continue;
        }

        let c = ch | 0x20;

        if c >= b'a' && c <= b'f' {
            value = value * 16 + (c - b'a' + 10) as i64;
            continue;
        }

        return -1;
    }

    value
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).is_ok() && !line.is_empty() {
        let trimmed = line.trim_end_matches(&['\r', '\n'][..]);
        let r = ngx_hextoi(trimmed.as_bytes());
        println!("{}", r);
    }
}
