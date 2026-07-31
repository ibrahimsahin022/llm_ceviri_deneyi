use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut line)) = lines.next() {
        // Remove trailing newlines/carriage returns
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }

        // Print reversed
        for i in (0..line.len()).rev() {
            print!("{}", line.chars().nth(i).unwrap());
        }
        println!();
    }
}
