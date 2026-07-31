use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut freq = [0; 26];
        for c in line.chars() {
            if c >= 'a' && c <= 'z' {
                freq[(c as usize) - ('a' as usize)] += 1;
            }
        }
        for i in 0..26 {
            if freq[i] > 0 {
                println!("{}:{}", (('a' as u8 + i as u8) as char), freq[i]);
            }
        }
    }
}
