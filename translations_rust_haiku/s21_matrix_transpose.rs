use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (r, c): (usize, usize) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(0), parts[1].parse().unwrap_or(0))
        } else {
            (0, 0)
        }
    };

    let mut m = vec![vec![0i64; c]; r];
    for i in 0..r {
        for j in 0..c {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(val) = line.trim().parse::<i64>() {
                    m[i][j] = val;
                }
            }
        }
    }

    for j in 0..c {
        for i in 0..r {
            print!("{}", m[i][j]);
            if i < r - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
