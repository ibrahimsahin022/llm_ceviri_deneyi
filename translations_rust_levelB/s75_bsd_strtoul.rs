use std::io::{self, Read};

const ULONG_MAX: u32 = u32::MAX;

const ERR_OK: i32 = 0;
const ERR_RANGE: i32 = 1;
const ERR_INVAL: i32 = 2;

fn is_space(c: i32) -> bool {
    c == b' ' as i32 || (c >= 0x09 && c <= 0x0d)
}

fn is_digit(c: i32) -> bool {
    c >= b'0' as i32 && c <= b'9' as i32
}

fn is_alpha(c: i32) -> bool {
    (c >= b'A' as i32 && c <= b'Z' as i32) || (c >= b'a' as i32 && c <= b'z' as i32)
}

fn is_upper(c: i32) -> bool {
    c >= b'A' as i32 && c <= b'Z' as i32
}

fn is_xdigit(c: i32) -> bool {
    is_digit(c) || (c >= b'A' as i32 && c <= b'F' as i32) || (c >= b'a' as i32 && c <= b'f' as i32)
}

/// (acc, errno, endptr_index)
fn strtoul(nptr: &[u8], base_in: i32) -> (u32, i32, usize) {
    let mut base = base_in;
    let mut errno = ERR_OK;

    if base < 0 || base == 1 || base > 36 {
        return (0, ERR_INVAL, 0);
    }

    let mut buf = nptr.to_vec();
    buf.push(0);
    let at = |i: usize| -> i32 { *buf.get(i).unwrap_or(&0) as i32 };

    let mut s: usize = 0;
    let mut c: i32;
    loop {
        c = at(s);
        s += 1;
        if !is_space(c) {
            break;
        }
    }

    let neg;
    if c == b'-' as i32 {
        neg = true;
        c = at(s);
        s += 1;
    } else {
        neg = false;
        if c == b'+' as i32 {
            c = at(s);
            s += 1;
        }
    }

    if (base == 0 || base == 16) && c == b'0' as i32
        && (at(s) == b'x' as i32 || at(s) == b'X' as i32)
        && is_xdigit(at(s + 1))
    {
        c = at(s + 1);
        s += 2;
        base = 16;
    }
    if base == 0 {
        base = if c == b'0' as i32 { 8 } else { 10 };
    }

    let cutoff: u32 = ULONG_MAX / base as u32;
    let cutlim: i32 = (ULONG_MAX % base as u32) as i32;
    let mut acc: u32 = 0;
    let mut any: i32 = 0;

    loop {
        let mut d = c;
        if is_digit(d) {
            d -= b'0' as i32;
        } else if is_alpha(d) {
            d -= if is_upper(d) {
                b'A' as i32 - 10
            } else {
                b'a' as i32 - 10
            };
        } else {
            break;
        }
        if d >= base {
            break;
        }
        if any >= 0 {
            if acc > cutoff || (acc == cutoff && d > cutlim) {
                any = -1;
                acc = ULONG_MAX;
                errno = ERR_RANGE;
            } else {
                any = 1;
                acc = acc.wrapping_mul(base as u32);
                acc = acc.wrapping_add(d as u32);
            }
        }
        c = at(s);
        s += 1;
    }

    if neg && any > 0 {
        acc = acc.wrapping_neg();
    }
    let endptr = if any != 0 { s - 1 } else { 0 };
    (acc, errno, endptr)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let numstr = match it.next() {
            Some(s) => s,
            None => continue,
        };
        let base: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let (result, errno, consumed) = strtoul(numstr.as_bytes(), base);
        let errname = if errno == ERR_RANGE {
            "ERANGE"
        } else if errno == ERR_INVAL {
            "EINVAL"
        } else {
            "OK"
        };
        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}
