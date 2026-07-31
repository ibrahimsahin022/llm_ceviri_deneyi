use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(raw) = line.trim().parse::<u32>() {
            // Simulate C bitfields with masking
            let a = raw & 0x1;           // 1 bit
            let b = raw & 0x7;           // 3 bits
            let c = raw & 0xF;           // 4 bits
            println!("{} {} {}", a, b, c);
        }
    }
}
