use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, k): (i32, i32) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(-1), parts[1].parse().unwrap_or(-1))
        } else {
            (-1, -1)
        }
    };

    if n < 0 || k < 0 {
        return;
    }

    let mut arr = Vec::with_capacity(n as usize);
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(val) = line.trim().parse::<i32>() {
                arr.push(val);
            }
        }
    }

    let mut start = n - k;
    if start < 0 {
        start = 0;
    }

    let mut sum: i64 = 0;
    for i in start as usize..arr.len() {
        sum += arr[i] as i64;
    }

    println!("{}", sum);
}
