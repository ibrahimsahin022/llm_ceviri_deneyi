use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut hash: u32 = 5381;
        for ch in line.chars() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            hash = hash.wrapping_mul(33).wrapping_add(ch as u32);
        }
        println!("{}", hash);
    }
}
