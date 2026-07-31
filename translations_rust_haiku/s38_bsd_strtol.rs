use std::io::{self, BufRead};
use std::sync::Mutex;
use std::sync::OnceLock;

const LONG_MIN: i32 = -2147483648;
const LONG_MAX: i32 = 2147483647;
const EINVAL: i32 = 1;
const ERANGE: i32 = 2;

fn errno() -> &'static Mutex<i32> {
    static ERR: OnceLock<Mutex<i32>> = OnceLock::new();
    ERR.get_or_init(|| Mutex::new(0))
}

fn strtol(nptr: &str, base: i32) -> (i32, i32, usize) {
    *errno().lock().unwrap() = 0;

    if base < 0 || base == 1 || base > 36 {
        *errno().lock().unwrap() = EINVAL;
        return (0, EINVAL, 0);
    }

    let bytes = nptr.as_bytes();
    let mut s = 0usize;

    while s < bytes.len() && (bytes[s] as char).is_whitespace() {
        s += 1;
    }

    let mut neg = false;
    if s < bytes.len() && bytes[s] as char == '-' {
        neg = true;
        s += 1;
    } else if s < bytes.len() && bytes[s] as char == '+' {
        s += 1;
    }

    let mut c = if s < bytes.len() {
        bytes[s] as char
    } else {
        '\0'
    };

    if (base == 0 || base == 16)
        && c == '0'
        && s + 1 < bytes.len()
        && (bytes[s + 1] as char == 'x' || bytes[s + 1] as char == 'X')
    {
        if s + 2 < bytes.len() && (bytes[s + 2] as char).is_ascii_hexdigit() {
            s += 2;
            c = bytes[s] as char;
            s += 1;
        }
    }

    let base = if base == 0 {
        if c == '0' { 8 } else { 10 }
    } else {
        base
    };

    let cutoff = if neg { LONG_MIN } else { LONG_MAX };
    let mut cutlim = cutoff % base;
    let mut cutoff = cutoff / base;

    if neg {
        if cutlim > 0 {
            cutlim -= base;
            cutoff += 1;
        }
        cutlim = -cutlim;
    }

    let mut acc = 0i32;
    let mut any = 0;
    let start_s = s;

    while s < bytes.len() {
        let ch = bytes[s] as char;
        let digit = if ch.is_ascii_digit() {
            (ch as i32 - '0' as i32) as i32
        } else if ch.is_ascii_alphabetic() {
            if ch.is_ascii_uppercase() {
                (ch as i32 - 'A' as i32 + 10) as i32
            } else {
                (ch as i32 - 'a' as i32 + 10) as i32
            }
        } else {
            break;
        };

        if digit >= base {
            break;
        }

        if any < 0 {
            s += 1;
            continue;
        }

        if neg {
            if acc < cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = LONG_MIN;
                *errno().lock().unwrap() = ERANGE;
            } else {
                any = 1;
                acc = acc * base - digit;
            }
        } else {
            if acc > cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = LONG_MAX;
                *errno().lock().unwrap() = ERANGE;
            } else {
                any = 1;
                acc = acc * base + digit;
            }
        }
        s += 1;
    }

    let consumed = if any != 0 { s - start_s } else { 0 };
    (acc, *errno().lock().unwrap(), consumed)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(base) = parts[1].parse::<i32>() {
                    let (result, err, consumed) = strtol(parts[0], base);
                    let errname = match err {
                        2 => "ERANGE",
                        1 => "EINVAL",
                        _ => "OK",
                    };
                    println!("result={} errno={} consumed={}", result, errname, consumed);
                }
            }
        }
    }
}
