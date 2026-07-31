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

    let mut bytes = 0;
    let mut chars = 0;
    for c in line.as_bytes() {
        bytes += 1;
        if (c & 0xC0) != 0x80 {
            chars += 1;
        }
    }

    println!("{} {}", bytes, chars);
}
