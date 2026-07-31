use std::io::{self, Read};

fn luhn(cc: &[u8]) -> bool {
    let m: [i32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut odd = true;
    let mut sum: i32 = 0;

    let mut i = cc.len();
    while i > 0 {
        i -= 1;
        let digit = (cc[i] as i32) - ('0' as i32);
        if odd {
            sum += digit;
        } else {
            let idx = digit.rem_euclid(10) as usize;
            sum += m[idx];
        }
        odd = !odd;
    }

    sum % 10 == 0
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    if input.is_empty() {
        return;
    }
    let mut end = 0usize;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    let mut buf = input[..end].to_vec();

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    println!("{}", if luhn(&buf) { "ok" } else { "not ok" });
}
