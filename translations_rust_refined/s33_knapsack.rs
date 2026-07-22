use std::io::{self, Read};

struct Item {
    weight: usize,
    value: i64,
}

fn knapsack(items: &[Item], cap: usize) -> Vec<bool> {
    let n = items.len();
    let mut m = vec![vec![0i64; cap + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=cap {
            if items[i - 1].weight > j {
                m[i][j] = m[i - 1][j];
            } else {
                let a = m[i - 1][j];
                let b = m[i - 1][j - items[i - 1].weight] + items[i - 1].value;
                m[i][j] = if a > b { a } else { b };
            }
        }
    }

    let mut s = vec![false; n];
    let mut j = cap;
    for i in (1..=n).rev() {
        if m[i][j] > m[i - 1][j] {
            s[i - 1] = true;
            j -= items[i - 1].weight;
        }
    }
    s
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let cap: usize = it.next().unwrap().parse().unwrap();

    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let weight: usize = it.next().unwrap().parse().unwrap();
        let value: i64 = it.next().unwrap().parse().unwrap();
        items.push(Item { weight, value });
    }

    let s = knapsack(&items, cap);

    let mut tw: usize = 0;
    let mut tv: i64 = 0;
    for i in 0..n {
        if s[i] {
            tw += items[i].weight;
            tv += items[i].value;
        }
    }
    println!("weight={} value={}", tw, tv);
}
