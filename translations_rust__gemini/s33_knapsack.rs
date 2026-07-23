use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Item {
    weight: i32,
    value: i32,
}

fn knapsack(items: &[Item], n: usize, w: usize) -> Vec<i32> {
    let mut mm = vec![0i32; (n + 1) * (w + 1)];

    for i in 1..=n {
        for j in 0..=w {
            if items[i - 1].weight > j as i32 {
                mm[i * (w + 1) + j] = mm[(i - 1) * (w + 1) + j];
            } else {
                let a = mm[(i - 1) * (w + 1) + j];
                let b_idx = j as i32 - items[i - 1].weight;
                let b = mm[(i - 1) * (w + 1) + b_idx as usize] + items[i - 1].value;
                mm[i * (w + 1) + j] = if a > b { a } else { b };
            }
        }
    }

    let mut s = vec![0i32; n];
    let mut j = w;
    for i in (1..=n).rev() {
        if mm[i * (w + 1) + j] > mm[(i - 1) * (w + 1) + j] {
            s[i - 1] = 1;
            j -= items[i - 1].weight as usize;
        }
    }

    s
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };
    let cap: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    if n < 0 || cap < 0 {
        return;
    }

    let n_usize = n as usize;
    let cap_usize = cap as usize;

    let mut items = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let weight: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        let value: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        items.push(Item { weight, value });
    }

    let s = knapsack(&items, n_usize, cap_usize);

    let mut tw = 0i32;
    let mut tv = 0i32;
    for i in 0..n_usize {
        if s[i] != 0 {
            tw += items[i].weight;
            tv += items[i].value;
        }
    }

    println!("weight={} value={}", tw, tv);
}
