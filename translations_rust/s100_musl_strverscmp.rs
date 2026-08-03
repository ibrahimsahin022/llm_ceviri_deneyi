use std::io::{self, BufRead};

fn strverscmp(l0: &str, r0: &str) -> i32 {
    let l = l0.as_bytes();
    let r = r0.as_bytes();
    let mut i = 0usize;
    let mut dp = 0usize;
    let mut z = true;

    loop {
        let lc = if i < l.len() { l[i] } else { 0 };
        let rc = if i < r.len() { r[i] } else { 0 };
        if lc != rc {
            break;
        }
        if lc == 0 {
            return 0;
        }
        if !lc.is_ascii_digit() {
            dp = i + 1;
            z = true;
        } else if lc != b'0' {
            z = false;
        }
        i += 1;
    }

    let lc_dp = if dp < l.len() { l[dp] } else { 0 };
    let rc_dp = if dp < r.len() { r[dp] } else { 0 };

    let ldig = (lc_dp.wrapping_sub(b'1')) < 9;
    let rdig = (rc_dp.wrapping_sub(b'1')) < 9;

    if ldig && rdig {
        let mut j = i;
        loop {
            let lj = if j < l.len() { l[j] } else { 0 };
            if !(lj as char).is_ascii_digit() {
                break;
            }
            let rj = if j < r.len() { r[j] } else { 0 };
            if !(rj as char).is_ascii_digit() {
                return 1;
            }
            j += 1;
        }
        let rj = if j < r.len() { r[j] } else { 0 };
        if (rj as char).is_ascii_digit() {
            return -1;
        }
    } else if z && dp < i {
        let lc_i = if i < l.len() { l[i] } else { 0 };
        let rc_i = if i < r.len() { r[i] } else { 0 };
        if (lc_i as char).is_ascii_digit() || (rc_i as char).is_ascii_digit() {
            return (lc_i as i32 - b'0' as i32) - (rc_i as i32 - b'0' as i32);
        }
    }

    let lc_i = if i < l.len() { l[i] as i32 } else { 0 };
    let rc_i = if i < r.len() { r[i] as i32 } else { 0 };
    lc_i - rc_i
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let left = lines.next().unwrap().unwrap();
    let right = lines.next().unwrap().unwrap();
    println!("{}", strverscmp(&left, &right));
}
