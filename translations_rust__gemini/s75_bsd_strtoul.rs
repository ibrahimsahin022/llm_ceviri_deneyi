use std::io::{self, BufRead};
use std::os::raw::c_ulong;

const EINVAL: i32 = 22;
const ERANGE: i32 = 34;

fn parse_c_int(s: &str) -> Option<i32> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let mut idx = 0;
    let neg = if bytes[idx] == b'-' {
        idx += 1;
        true
    } else if bytes[idx] == b'+' {
        idx += 1;
        false
    } else {
        false
    };
    let start_digits = idx;
    let mut val: i32 = 0;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        val = val.wrapping_mul(10).wrapping_add((bytes[idx] - b'0') as i32);
        idx += 1;
    }
    if idx == start_digits {
        return None;
    }
    if neg {
        val = val.wrapping_neg();
    }
    Some(val)
}

fn strtoul(nptr: &[u8], base_in: i32) -> (c_ulong, usize, i32) {
    if base_in < 0 || base_in == 1 || base_in > 36 {
        return (0, 0, EINVAL);
    }

    let mut idx = 0;
    let mut get_byte = || {
        if idx < nptr.len() {
            let b = nptr[idx];
            idx += 1;
            b
        } else {
            0
        }
    };

    let mut c = get_byte();
    while c != 0 && c.is_ascii_whitespace() {
        c = get_byte();
    }

    let neg = if c == b'-' {
        c = get_byte();
        true
    } else {
        if c == b'+' {
            c = get_byte();
        }
        false
    };

    let mut base = base_in;
    if (base == 0 || base == 16) && c == b'0' {
        let s0 = if idx < nptr.len() { nptr[idx] } else { 0 };
        let s1 = if idx + 1 < nptr.len() { nptr[idx + 1] } else { 0 };
        if (s0 == b'x' || s0 == b'X') && s1.is_ascii_hexdigit() {
            c = s1;
            idx += 2;
            base = 16;
        }
    }

    if base == 0 {
        base = if c == b'0' { 8 } else { 10 };
    }

    let cutoff = c_ulong::MAX / (base as c_ulong);
    let cutlim = c_ulong::MAX % (base as c_ulong);
    let mut acc: c_ulong = 0;
    let mut any: i32 = 0;
    let mut err: i32 = 0;

    loop {
        let digit: c_ulong = if c >= b'0' && c <= b'9' {
            (c - b'0') as c_ulong
        } else if c >= b'a' && c <= b'z' {
            (c - b'a' + 10) as c_ulong
        } else if c >= b'A' && c <= b'Z' {
            (c - b'A' + 10) as c_ulong
        } else {
            break;
        };

        if digit >= base as c_ulong {
            break;
        }

        if any < 0 {
            // continue consuming digits after overflow
        } else if acc > cutoff || (acc == cutoff && digit > cutlim) {
            any = -1;
            acc = c_ulong::MAX;
            err = ERANGE;
        } else {
            any = 1;
            acc = acc * (base as c_ulong) + digit;
        }

        c = get_byte();
    }

    if neg && any > 0 {
        acc = acc.wrapping_neg();
    }

    let consumed = if any != 0 { idx - 1 } else { 0 };
    (acc, consumed, err)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let mut words = line.split_whitespace();
        let numstr_full = match words.next() {
            Some(w) => w,
            None => continue,
        };
        let base_str = match words.next() {
            Some(b) => b,
            None => continue,
        };

        let base = match parse_c_int(base_str) {
            Some(b) => b,
            None => continue,
        };

        let numstr = if numstr_full.len() > 399 {
            &numstr_full[..399]
        } else {
            numstr_full
        };

        let (result, consumed, err) = strtoul(numstr.as_bytes(), base);
        let errname = if err == ERANGE {
            "ERANGE"
        } else if err == EINVAL {
            "EINVAL"
        } else {
            "OK"
        };

        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}
