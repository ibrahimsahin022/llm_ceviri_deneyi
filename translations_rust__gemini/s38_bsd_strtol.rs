use std::io::{self, BufRead};
use std::os::raw::{c_int, c_long};

fn strtol(bytes: &[u8], base_in: c_int) -> (c_long, &'static str, usize) {
    let mut base = base_in;

    if base < 0 || base == 1 || base > 36 {
        return (0, "EINVAL", 0);
    }

    let mut s_idx = 0;
    let mut c: u8;

    loop {
        c = if s_idx < bytes.len() { bytes[s_idx] } else { 0 };
        s_idx += 1;
        if !(c as char).is_ascii_whitespace() {
            break;
        }
    }

    let neg;
    if c == b'-' {
        neg = true;
        c = if s_idx < bytes.len() { bytes[s_idx] } else { 0 };
        s_idx += 1;
    } else {
        neg = false;
        if c == b'+' {
            c = if s_idx < bytes.len() { bytes[s_idx] } else { 0 };
            s_idx += 1;
        }
    }

    let peek0 = if s_idx < bytes.len() { bytes[s_idx] } else { 0 };
    let peek1 = if s_idx + 1 < bytes.len() { bytes[s_idx + 1] } else { 0 };

    if (base == 0 || base == 16)
        && c == b'0'
        && (peek0 == b'x' || peek0 == b'X')
        && (peek1 as char).is_ascii_hexdigit()
    {
        c = peek1;
        s_idx += 2;
        base = 16;
    }

    if base == 0 {
        base = if c == b'0' { 8 } else { 10 };
    }

    let mut cutoff: c_long = if neg { c_long::MIN } else { c_long::MAX };
    let mut cutlim: c_long = cutoff % (base as c_long);
    cutoff /= base as c_long;

    if neg {
        if cutlim > 0 {
            cutlim -= base as c_long;
            cutoff += 1;
        }
        cutlim = -cutlim;
    }

    let mut acc: c_long = 0;
    let mut any: i32 = 0;
    let mut err = "OK";

    loop {
        let digit: c_long = if (c as char).is_ascii_digit() {
            (c - b'0') as c_long
        } else if (c as char).is_ascii_alphabetic() {
            if (c as char).is_ascii_uppercase() {
                (c - b'A') as c_long + 10
            } else {
                (c - b'a') as c_long + 10
            }
        } else {
            break;
        };

        if digit >= (base as c_long) {
            break;
        }

        if any < 0 {
            // Keep consuming valid digits after overflow
        } else if neg {
            if acc < cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = c_long::MIN;
                err = "ERANGE";
            } else {
                any = 1;
                acc *= base as c_long;
                acc -= digit;
            }
        } else {
            if acc > cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = c_long::MAX;
                err = "ERANGE";
            } else {
                any = 1;
                acc *= base as c_long;
                acc += digit;
            }
        }

        c = if s_idx < bytes.len() { bytes[s_idx] } else { 0 };
        s_idx += 1;
    }

    let consumed = if any != 0 { s_idx - 1 } else { 0 };
    (acc, err, consumed)
}

fn parse_sscanf(line: &str) -> Option<(&str, c_int)> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return None;
    }

    let first_ws = trimmed.find(|c: char| c.is_whitespace()).unwrap_or(trimmed.len());
    let token1_full = &trimmed[..first_ws];
    let rest_after_token1 = &trimmed[first_ws..];

    let token1_len = token1_full.chars().take(399).map(|c| c.len_utf8()).sum::<usize>();
    let numstr = &token1_full[..token1_len];

    let remaining = format!("{}{}", &token1_full[token1_len..], rest_after_token1);
    let rest_trimmed = remaining.trim_start();
    if rest_trimmed.is_empty() {
        return None;
    }

    let bytes = rest_trimmed.as_bytes();
    let mut idx = 0;
    if idx < bytes.len() && (bytes[idx] == b'+' || bytes[idx] == b'-') {
        idx += 1;
    }
    let start_digits = idx;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
    }
    if idx == start_digits {
        return None;
    }

    let int_str = &rest_trimmed[..idx];
    let base: c_int = int_str.parse().ok()?;

    Some((numstr, base))
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    while handle.read_line(&mut line).unwrap_or(0) > 0 {
        if let Some((numstr, base)) = parse_sscanf(&line) {
            let (result, errname, consumed) = strtol(numstr.as_bytes(), base);
            println!("result={} errno={} consumed={}", result, errname, consumed);
        }
        line.clear();
    }
}
