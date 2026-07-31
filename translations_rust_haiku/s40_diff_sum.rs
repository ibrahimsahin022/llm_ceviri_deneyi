use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut arr: Vec<i32> = Vec::with_capacity(n.max(0) as usize);
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(val) = line.trim().parse::<i32>() {
                arr.push(val);
            }
        }
    }

    let mut diffsum = 0i32;
    for i in 0..n.max(0) as usize {
        if i + 1 < arr.len() {
            diffsum += arr[i + 1] - arr[i];
        }
    }

    println!("{}", diffsum);
}
