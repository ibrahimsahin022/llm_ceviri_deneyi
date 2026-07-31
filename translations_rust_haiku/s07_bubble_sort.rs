use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let n = std::cmp::min(n as usize, 1000);
    let mut a: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(val) = line.trim().parse::<i32>() {
                a.push(val);
            }
        }
    }

    // Bubble sort
    for i in 0..n.saturating_sub(1) {
        for j in 0..n.saturating_sub(1).saturating_sub(i) {
            if j + 1 < a.len() && a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }

    for (i, &val) in a.iter().enumerate() {
        print!("{}", val);
        if i < a.len() - 1 {
            print!(" ");
        }
    }
    println!();
}
