use std::io::{self, Read};

fn is_space_c(c: i32) -> bool {
    c == b' ' as i32
        || c == b'\t' as i32
        || c == b'\n' as i32
        || c == 0x0B
        || c == 0x0C
        || c == b'\r' as i32
}

fn is_digit_c(c: i32) -> bool {
    c >= b'0' as i32 && c <= b'9' as i32
}

fn is_alpha_c(c: i32) -> bool {
    (c >= b'a' as i32 && c <= b'z' as i32) || (c >= b'A' as i32 && c <= b'Z' as i32)
}

fn is_upper_c(c: i32) -> bool {
    c >= b'A' as i32 && c <= b'Z' as i32
}

fn is_xdigit(b: u8) -> bool {
    (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}

/// Returns (result, consumed_bytes, errno) where errno: 0=OK, 1=EINVAL, 2=ERANGE
fn strtol_bsd(nptr: &[u8], base_in: i32) -> (i32, usize, i32) {
    let mut base = base_in;
    if base < 0 || base == 1 || base > 36 {
        return (0, 0, 1);
    }

    let n = nptr.len();
    let mut idx = 0usize;
    let mut c: i32;

    loop {
        c = if idx < n { nptr[idx] as i32 } else { 0 };
        idx += 1;
        if !is_space_c(c) {
            break;
        }
    }

    let neg;
    if c == b'-' as i32 {
        neg = true;
        c = if idx < n { nptr[idx] as i32 } else { 0 };
        idx += 1;
    } else {
        neg = false;
        if c == b'+' as i32 {
            c = if idx < n { nptr[idx] as i32 } else { 0 };
            idx += 1;
        }
    }

    if (base == 0 || base == 16) && c == b'0' as i32 {
        let s0 = if idx < n { nptr[idx] } else { 0 };
        let s1 = if idx + 1 < n { nptr[idx + 1] } else { 0 };
        if (s0 == b'x' || s0 == b'X') && is_xdigit(s1) {
            c = s1 as i32;
            idx += 2;
            base = 16;
        }
    }
    if base == 0 {
        base = if c == b'0' as i32 { 8 } else { 10 };
    }

    let cutoff0: i64 = if neg { i32::MIN as i64 } else { i32::MAX as i64 };
    let mut cutlim = (cutoff0 % base as i64) as i32;
    let mut cutoff = (cutoff0 / base as i64) as i32;
    if neg {
        if cutlim > 0 {
            cutlim -= base;
            cutoff += 1;
        }
        cutlim = -cutlim;
    }

    let mut acc: i32 = 0;
    let mut any: i32 = 0;
    let mut errno_val = 0;

    loop {
        let mut cc = c;
        if is_digit_c(cc) {
            cc -= b'0' as i32;
        } else if is_alpha_c(cc) {
            cc -= if is_upper_c(cc) {
                b'A' as i32 - 10
            } else {
                b'a' as i32 - 10
            };
        } else {
            break;
        }
        if cc >= base {
            break;
        }
        if any < 0 {
            // consumed but overflowed already; skip accumulation
        } else if neg {
            if acc < cutoff || (acc == cutoff && cc > cutlim) {
                any = -1;
                acc = i32::MIN;
                errno_val = 2;
            } else {
                any = 1;
                acc = acc.wrapping_mul(base).wrapping_sub(cc);
            }
        } else {
            if acc > cutoff || (acc == cutoff && cc > cutlim) {
                any = -1;
                acc = i32::MAX;
                errno_val = 2;
            } else {
                any = 1;
                acc = acc.wrapping_mul(base).wrapping_add(cc);
            }
        }
        c = if idx < n { nptr[idx] as i32 } else { 0 };
        idx += 1;
    }

    let consumed = if any != 0 { idx - 1 } else { 0 };
    (acc, consumed, errno_val)
}

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;
    let total_len = input.len();

    loop {
        if pos >= total_len {
            break;
        }
        // fgets: read one line (up to and including '\n', capped at 511 data bytes)
        let start = pos;
        let maxdata = 511usize;
        let mut count = 0usize;
        let mut end = pos;
        while end < total_len && count < maxdata {
            if input[end] == b'\n' {
                end += 1;
                break;
            }
            end += 1;
            count += 1;
        }
        pos = end;
        let line = &input[start..end];

        // sscanf(line, "%399s %d", numstr, &base)
        let mut sp = 0usize;
        let ln = line.len();

        // %s: skip leading whitespace, read non-whitespace up to 399 chars
        while sp < ln && is_space_ws(line[sp]) {
            sp += 1;
        }
        let word_start = sp;
        let mut wcount = 0usize;
        while sp < ln && !is_space_ws(line[sp]) && wcount < 399 {
            sp += 1;
            wcount += 1;
        }
        if word_start == sp {
            // no word matched
            continue;
        }
        let numstr = line[word_start..sp].to_vec();

        // %d: skip leading whitespace, optional sign, digits
        while sp < ln && is_space_ws(line[sp]) {
            sp += 1;
        }
        let sign_start = sp;
        if sp < ln && (line[sp] == b'+' || line[sp] == b'-') {
            sp += 1;
        }
        let digits_start = sp;
        while sp < ln && line[sp].is_ascii_digit() {
            sp += 1;
        }
        if digits_start == sp {
            // no integer matched
            continue;
        }
        let base_str = String::from_utf8_lossy(&line[sign_start..sp]).into_owned();
        let base: i32 = match base_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let (result, consumed, errno_val) = strtol_bsd(&numstr, base);
        let errname = match errno_val {
            2 => "ERANGE",
            1 => "EINVAL",
            _ => "OK",
        };

        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}
