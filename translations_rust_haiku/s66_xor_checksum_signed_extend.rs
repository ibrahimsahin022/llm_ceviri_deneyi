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

    let mut acc: u32 = 0;
    for c in line.as_bytes() {
        let c_signed = *c as i8 as i32;
        acc ^= c_signed as u32;
    }

    println!("{}", acc);
}
