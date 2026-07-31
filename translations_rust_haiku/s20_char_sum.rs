use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut sum: i64 = 0;
        for c in line.chars() {
            if c == '\n' || c == '\r' {
                break;
            }
            // Cast to signed byte to match C behavior
            let byte = c as u8 as i8 as i64;
            sum += byte;
        }
        println!("{}", sum);
    }
}
