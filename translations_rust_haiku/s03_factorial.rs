use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            let mut f: u64 = 1;
            for i in 2..=n {
                f *= i as u64;
            }
            println!("{}", f);
        }
    }
}
