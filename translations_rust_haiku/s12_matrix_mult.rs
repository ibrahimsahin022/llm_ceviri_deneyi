use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (r1, c1): (usize, usize) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(0), parts[1].parse().unwrap_or(0))
        } else {
            (0, 0)
        }
    };

    let mut a = vec![vec![0i64; c1]; r1];
    for i in 0..r1 {
        for j in 0..c1 {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(val) = line.trim().parse::<i64>() {
                    a[i][j] = val;
                }
            }
        }
    }

    let (r2, c2): (usize, usize) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(0), parts[1].parse().unwrap_or(0))
        } else {
            (0, 0)
        }
    };

    let mut b = vec![vec![0i64; c2]; r2];
    for i in 0..r2 {
        for j in 0..c2 {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(val) = line.trim().parse::<i64>() {
                    b[i][j] = val;
                }
            }
        }
    }

    let mut c = vec![vec![0i64; c2]; r1];
    for i in 0..r1 {
        for j in 0..c2 {
            let mut s: i64 = 0;
            for k in 0..c1 {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s;
        }
    }

    for i in 0..r1 {
        for j in 0..c2 {
            print!("{}", c[i][j]);
            if j < c2 - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
