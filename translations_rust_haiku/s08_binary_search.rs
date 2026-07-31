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

    let target: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut lo = 0i32;
    let mut hi = (n as i32) - 1;
    let mut ans = -1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mid_idx = mid as usize;
        if a[mid_idx] == target {
            ans = mid;
            break;
        } else if a[mid_idx] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", ans);
}
