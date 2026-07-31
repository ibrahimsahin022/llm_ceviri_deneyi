use std::io::{self, Read};

struct Item {
    weight: i32,
    value: i32,
}

fn knapsack(items: &[Item], n: usize, w: usize) -> Vec<i32> {
    let mut m = vec![vec![0i32; w + 1]; n + 1];
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

    let mut s = vec![0i32; n];
    let mut j = w as i64;
    let mut i = n as i64;
    while i > 0 {
        if m[i as usize][j as usize] > m[(i - 1) as usize][j as usize] {
            s[(i - 1) as usize] = 1;
            j -= items[(i - 1) as usize].weight as i64;
        }
        i -= 1;
    }
    s
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let cap: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut items: Vec<Item> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let weight: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let value: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        items.push(Item { weight, value });
    }

    let s = knapsack(&items, n as usize, cap as usize);

    let mut tw: i32 = 0;
    let mut tv: i32 = 0;
    for i in 0..(n as usize) {
        if s[i] != 0 {
            tw += items[i].weight;
            tv += items[i].value;
        }
    }

    println!("weight={} value={}", tw, tv);
}
