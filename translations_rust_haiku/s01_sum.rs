use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut sum: i64 = 0;
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(x) = line.trim().parse::<i64>() {
                sum += x;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!("{}", sum);
}
