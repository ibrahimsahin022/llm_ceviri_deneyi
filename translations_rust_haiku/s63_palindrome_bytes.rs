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

    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut ok = true;

    for i in 0..len/2 {
        if bytes[i] != bytes[len - 1 - i] {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "EVET" } else { "HAYIR" });
}
