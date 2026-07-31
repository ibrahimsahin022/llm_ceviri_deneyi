use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    let mut alpha = 0;
    let mut digit = 0;

    for c in line.as_bytes() {
        let c_signed = *c as i8 as u8 as char;
        if c_signed.is_alphabetic() {
            alpha += 1;
        }
        if c_signed.is_numeric() {
            digit += 1;
        }
    }

    println!("{} {}", alpha, digit);
}
