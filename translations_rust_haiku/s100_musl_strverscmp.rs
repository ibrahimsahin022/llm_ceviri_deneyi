use std::io::{self, BufRead};

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn my_strverscmp(l0: &[u8], r0: &[u8]) -> i32 {
    let mut i = 0;
    let mut dp = 0;
    let mut z = 1;

    // Skip matching prefix
    loop {
        let l_c = if i < l0.len() { l0[i] } else { 0 };
        let r_c = if i < r0.len() { r0[i] } else { 0 };

        if l_c != r_c {
            break;
        }

        let c = l_c;
        if c == 0 {
            return 0;
        }

        if !is_digit(c) {
            dp = i + 1;
            z = 1;
        } else if c != b'0' {
            z = 0;
        }

        i += 1;
    }

    let l_i = if i < l0.len() { l0[i] } else { 0 };
    let r_i = if i < r0.len() { r0[i] } else { 0 };
    let l_dp = if dp < l0.len() { l0[dp] } else { 0 };
    let r_dp = if dp < r0.len() { r0[dp] } else { 0 };

    if l_dp.wrapping_sub(b'1') < 9 && r_dp.wrapping_sub(b'1') < 9 {
        let mut j = i;
        loop {
            let l_j = if j < l0.len() { l0[j] } else { 0 };
            let r_j = if j < r0.len() { r0[j] } else { 0 };

            if !is_digit(l_j) {
                break;
            }
            if !is_digit(r_j) {
                return 1;
            }
            j += 1;
        }

        let r_j = if j < r0.len() { r0[j] } else { 0 };
        if is_digit(r_j) {
            return -1;
        }
    } else if z > 0 && dp < i && (is_digit(l_i) || is_digit(r_i)) {
        let l_diff = if l_i >= b'0' && l_i <= b'9' {
            (l_i - b'0') as i32
        } else {
            0
        };
        let r_diff = if r_i >= b'0' && r_i <= b'9' {
            (r_i - b'0') as i32
        } else {
            0
        };
        return l_diff - r_diff;
    }

    (l_i as i32) - (r_i as i32)
}

fn main() {
    let stdin = io::stdin();
    let mut left = String::new();
    let mut right = String::new();

    if stdin.read_line(&mut left).is_err() {
        return;
    }
    if stdin.read_line(&mut right).is_err() {
        return;
    }

    left = left.trim_end().to_string();
    right = right.trim_end().to_string();

    let result = my_strverscmp(left.as_bytes(), right.as_bytes());
    println!("{}", result);
}
