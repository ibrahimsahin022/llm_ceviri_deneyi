use std::io::{self, BufRead};

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut d = 2;
    while (d as i64) * (d as i64) <= x as i64 {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            let mut count = 0;
            for i in 2..=n {
                if is_prime(i) {
                    count += 1;
                }
            }
            println!("{}", count);
        }
    }
}
