use std::io::{self, BufRead};

fn my_strverscmp(l0: &[u8], r0: &[u8]) -> i32 {
    let get_l = |idx: usize| -> u8 { l0.get(idx).copied().unwrap_or(0) };
    let get_r = |idx: usize| -> u8 { r0.get(idx).copied().unwrap_or(0) };

    let mut i = 0;
    let mut dp = 0;
    let mut z = 1;

    while get_l(i) == get_r(i) {
        let c = get_l(i);
        if c == 0 {
            return 0;
        }
        if !c.is_ascii_digit() {
            dp = i + 1;
            z = 1;
        } else if c != b'0' {
            z = 0;
        }
        i += 1;
    }

    let l_dp = get_l(dp);
    let r_dp = get_r(dp);

    if l_dp.wrapping_sub(b'1') < 9 && r_dp.wrapping_sub(b'1') < 9 {
        let mut j = i;
        while get_l(j).is_ascii_digit() {
            if !get_r(j).is_ascii_digit() {
                return 1;
            }
            j += 1;
        }
        if get_r(j).is_ascii_digit() {
            return -1;
        }
    } else if z != 0 && dp < i && (get_l(i).is_ascii_digit() || get_r(i).is_ascii_digit()) {
        return (get_l(i).wrapping_sub(b'0')) as i32 - (get_r(i).wrapping_sub(b'0')) as i32;
    }

    (get_l(i) as i32) - (get_r(i) as i32)
}

fn fgets_like<R: BufRead>(reader: &mut R, max_bytes: usize) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut count = 0;
    while count < max_bytes {
        let available = match reader.fill_buf() {
            Ok(b) if !b.is_empty() => b,
            _ => break,
        };
        let mut consumed = 0;
        for &b in available {
            if count >= max_bytes {
                break;
            }
            buf.push(b);
            consumed += 1;
            count += 1;
            if b == b'\n' {
                break;
            }
        }
        reader.consume(consumed);
        if buf.last() == Some(&b'\n') {
            break;
        }
    }

    if buf.is_empty() {
        None
    } else {
        let len = buf.iter().position(|&b| b == b'\r' || b == b'\n' || b == 0).unwrap_or(buf.len());
        buf.truncate(len);
        Some(buf)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let left = match fgets_like(&mut handle, 255) {
        Some(l) => l,
        None => return,
    };

    let right = match fgets_like(&mut handle, 255) {
        Some(r) => r,
        None => return,
    };

    println!("{}", my_strverscmp(&left, &right));
}
