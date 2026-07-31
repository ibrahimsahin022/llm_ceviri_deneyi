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

    let mut hash: u32 = 0;
    for c in line.as_bytes() {
        let c_u32 = *c as u32;
        hash = c_u32.wrapping_add(hash.wrapping_shl(6)).wrapping_add(hash.wrapping_shl(16)).wrapping_sub(hash);
    }

    println!("{}", hash);
}
