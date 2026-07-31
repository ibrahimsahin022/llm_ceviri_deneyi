use std::io::{self, BufRead};

fn luhn(cc: &str) -> bool {
    let m = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut odd = true;
    let mut sum = 0;

    for c in cc.chars().rev() {
        let digit = (c as u32 - '0' as u32) as usize;
        if digit < 10 {
            sum += if odd {
                digit
            } else {
                m[digit]
            };
            odd = !odd;
        }
    }

    sum % 10 == 0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut line)) = lines.next() {
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }
        println!("{}", if luhn(&line) { "ok" } else { "not ok" });
    }
}
