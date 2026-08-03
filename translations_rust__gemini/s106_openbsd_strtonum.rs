use std::io::{self, Read};

fn my_strtonum(numstr: &[u8], minval: i64, maxval: i64) -> (i64, &'static str) {
    if minval > maxval {
        return (0, "invalid");
    }

    let mut idx = 0;

    while idx < numstr.len()
        && (numstr[idx] == b' '
            || numstr[idx] == b'\t'
            || numstr[idx] == b'\n'
            || numstr[idx] == 11
            || numstr[idx] == 12
            || numstr[idx] == b'\r')
    {
        idx += 1;
    }

    let mut is_neg = false;
    if idx < numstr.len() {
        if numstr[idx] == b'-' {
            is_neg = true;
            idx += 1;
        } else if numstr[idx] == b'+' {
            idx += 1;
        }
    }

    let digit_start = idx;
    let mut val: i64 = 0;
    let mut overflow = false;

    while idx < numstr.len() && numstr[idx].is_ascii_digit() {
        let d = (numstr[idx] - b'0') as i64;
        if is_neg {
            if let Some(res) = val.checked_mul(10).and_then(|v| v.checked_sub(d)) {
                val = res;
            } else {
                overflow = true;
            }
        } else {
            if let Some(res) = val.checked_mul(10).and_then(|v| v.checked_add(d)) {
                val = res;
            } else {
                overflow = true;
            }
        }
        idx += 1;
    }

    if idx == digit_start || idx < numstr.len() {
        return (0, "invalid");
    }

    if (overflow && is_neg) || val < minval {
        return (0, "too small");
    }

    if (overflow && !is_neg) || val > maxval {
        return (0, "too large");
    }

    (val, "yok")
}

fn main() {
    let mut stdin = io::stdin().lock();
    let mut buffer = Vec::new();
    if stdin.read_to_end(&mut buffer).is_err() || buffer.is_empty() {
        return;
    }

    let mut line1_len = 0;
    while line1_len < buffer.len() && line1_len < 63 {
        let b = buffer[line1_len];
        line1_len += 1;
        if b == b'\n' {
            break;
        }
    }

    if line1_len == 0 {
        return;
    }

    let line1_bytes = &buffer[..line1_len];
    let rest_bytes = &buffer[line1_len..];

    let mut end = line1_bytes.len();
    for (i, &b) in line1_bytes.iter().enumerate() {
        if b == b'\r' || b == b'\n' {
            end = i;
            break;
        }
    }
    let numstr = &line1_bytes[..end];

    let rest_str = match std::str::from_utf8(rest_bytes) {
        Ok(s) => s,
        Err(_) => "",
    };

    let mut tokens = rest_str.split_whitespace();
    let minval: i64 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let maxval: i64 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let (v, errstr) = my_strtonum(numstr, minval, maxval);
    println!("value={} err={}", v, errstr);
}
