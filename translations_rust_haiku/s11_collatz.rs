use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(mut n) = line.trim().parse::<u64>() {
            let mut steps: u64 = 0;
            while n != 1 {
                if n % 2 == 0 {
                    n = n / 2;
                } else {
                    n = 3 * n + 1;
                }
                steps += 1;
            }
            println!("{}", steps);
        }
    }
}
