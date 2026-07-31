use std::io;

const ULONG_MAX: u32 = 4294967295u32;
const EINVAL: i32 = 22;
const ERANGE: i32 = 34;

thread_local! {
    static ERRNO: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
}

fn set_errno(val: i32) {
    ERRNO.with(|e| {
        *e.borrow_mut() = val;
    });
}

fn get_errno() -> i32 {
    ERRNO.with(|e| {
        *e.borrow()
    });
}

fn char_to_digit(c: u8) -> Option<u32> {
    if c >= b'0' && c <= b'9' {
        Some((c - b'0') as u32)
    } else if c >= b'A' && c <= b'Z' {
        Some((c - b'A' + 10) as u32)
    } else if c >= b'a' && c <= b'z' {
        Some((c - b'a' + 10) as u32)
    } else {
        None
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_alpha(c: u8) -> bool {
    (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
}

fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == b'\x0B' || c == b'\x0C'
}

fn is_xdigit(c: u8) -> bool {
    is_digit(c) || (c >= b'A' && c <= b'F') || (c >= b'a' && c <= b'f')
}

fn is_upper(c: u8) -> bool {
    c >= b'A' && c <= b'Z'
}

fn strtoul(nptr: &str, base: i32) -> (u32, usize) {
    let bytes = nptr.as_bytes();
    let mut s = 0;

    if base < 0 || base == 1 || base > 36 {
        set_errno(EINVAL);
        return (0, 0);
    }

    while s < bytes.len() && is_space(bytes[s]) {
        s += 1;
    }

    let start_pos = s;

    let mut neg = false;
    if s < bytes.len() && bytes[s] == b'-' {
        neg = true;
        s += 1;
    } else if s < bytes.len() && bytes[s] == b'+' {
        s += 1;
    }

    if (base == 0 || base == 16) && s < bytes.len() && bytes[s] == b'0' {
        if s + 1 < bytes.len() && (bytes[s + 1] == b'x' || bytes[s + 1] == b'X') {
            if s + 2 < bytes.len() && is_xdigit(bytes[s + 2]) {
                s += 2;
            }
        }
    }

    let actual_base = if base == 0 {
        if s < bytes.len() && bytes[s] == b'0' { 8 } else { 10 }
    } else {
        base
    };

    let cutoff = ULONG_MAX / (actual_base as u32);
    let cutlim = ULONG_MAX % (actual_base as u32);
    let mut acc: u32 = 0;
    let mut any = 0;
    let digit_start = s;

    while s < bytes.len() {
        let c_opt = char_to_digit(bytes[s]);
        let c = match c_opt {
            Some(d) => d,
            None => break,
        };

        if c >= (actual_base as u32) {
            break;
        }

        if any < 0 {
            s += 1;
            continue;
        }

        if acc > cutoff || (acc == cutoff && c > cutlim) {
            any = -1;
            acc = ULONG_MAX;
            set_errno(ERANGE);
        } else {
            any = 1;
            acc = acc.wrapping_mul(actual_base as u32);
            acc = acc.wrapping_add(c);
        }

        s += 1;
    }

    if neg && any > 0 {
        acc = acc.wrapping_neg();
    }

    let consumed = if any != 0 { s } else { 0 };

    (acc, consumed)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let trimmed = line.trim_end_matches(|c: char| c == '\n' || c == '\r');
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() != 2 {
                continue;
            }

            let numstr = parts[0];
            let base: i32 = match parts[1].parse() {
                Ok(b) => b,
                Err(_) => continue,
            };

            set_errno(0);
            let (result, consumed) = strtoul(numstr, base);
            let errno = get_errno();
            let errname = match errno {
                ERANGE => "ERANGE",
                EINVAL => "EINVAL",
                _ => "OK",
            };

            println!("result={} errno={} consumed={}", result, errname, consumed);
        }
    }
}
