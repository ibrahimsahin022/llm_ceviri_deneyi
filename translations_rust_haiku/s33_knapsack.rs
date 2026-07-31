use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Item {
    weight: i32,
    value: i32,
}

fn knapsack(items: &[Item], n: usize, w: usize) -> Vec<i32> {
    let mut m = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            if items[i - 1].weight as usize > j {
                m[i][j] = m[i - 1][j];
            } else {
                let a = m[i - 1][j];
                let b = m[i - 1][j - items[i - 1].weight as usize] + items[i - 1].value;
                m[i][j] = if a > b { a } else { b };
            }
        }
    }

    let mut s = vec![0; n];
    let mut i = n;
    let mut j = w;
    while i > 0 {
        if m[i][j] > m[i - 1][j] {
            s[i - 1] = 1;
            j -= items[i - 1].weight as usize;
        }
        i -= 1;
    }

    s
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, cap): (usize, usize) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(0), parts[1].parse().unwrap_or(0))
        } else {
            (0, 0)
        }
    };

    let mut items = Vec::new();
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(w), Ok(v)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    items.push(Item { weight: w, value: v });
                }
            }
        }
    }

    let s = knapsack(&items, n, cap);

    let mut tw = 0;
    let mut tv = 0;
    for i in 0..n {
        if s[i] != 0 {
            tw += items[i].weight;
            tv += items[i].value;
        }
    }

    println!("weight={} value={}", tw, tv);
}
