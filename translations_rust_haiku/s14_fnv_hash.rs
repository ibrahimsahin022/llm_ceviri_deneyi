use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut hash: u32 = 2166136261u32;
        for c in line.chars() {
            if c == '\n' || c == '\r' {
                break;
            }
            hash ^= c as u32;
            hash = hash.wrapping_mul(16777619u32);
        }
        println!("{}", hash);
    }
}
