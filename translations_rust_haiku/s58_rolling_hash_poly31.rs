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

    let mut h: u32 = 0;
    for c in line.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(*c as u32);
    }

    println!("{}", h);
}
