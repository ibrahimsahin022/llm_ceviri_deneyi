use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut k: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    k = ((k % 26) + 26) % 26;

    if let Some(Ok(line)) = lines.next() {
        for c in line.chars() {
            if c >= 'a' && c <= 'z' {
                let shifted = ((c as i32 - 'a' as i32 + k) % 26) as u8 as char;
                print!("{}", ('a' as u8 as i32 + (c as i32 - 'a' as i32 + k) % 26) as u8 as char);
            } else if c >= 'A' && c <= 'Z' {
                print!("{}", ('A' as u8 as i32 + (c as i32 - 'A' as i32 + k) % 26) as u8 as char);
            } else if c != '\n' && c != '\r' {
                print!("{}", c);
            }
        }
        println!();
    }
}
