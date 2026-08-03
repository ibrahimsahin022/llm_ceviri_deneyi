use std::io::{self, Read};

const NGX_ERROR: i32 = -1;
const NGX_MAX_INT_T_VALUE: i32 = i32::MAX;

fn ngx_hextoi(line: &[u8]) -> i32 {
    if line.is_empty() {
        return NGX_ERROR;
    }

    let cutoff = NGX_MAX_INT_T_VALUE / 16;
    let mut value: i32 = 0;

    for &ch in line {
        if value > cutoff {
            return NGX_ERROR;
        }

        if ch >= b'0' && ch <= b'9' {
            value = value * 16 + (ch - b'0') as i32;
            continue;
        }

        let c = ch | 0x20;
        if c >= b'a' && c <= b'f' {
            value = value * 16 + (c - b'a' + 10) as i32;
            continue;
        }

        return NGX_ERROR;
    }

    value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = match input.lines().next() {
        Some(l) => l,
        None => return,
    };
    println!("{}", ngx_hextoi(line.as_bytes()));
}
