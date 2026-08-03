use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n < 2 || n > 1000 {
        return;
    }
    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr.push(v);
    }
    let mut count = 0;
    for i in 0..n {
        let prev_idx = i - 1;
        let next_idx = i + 1;
        if prev_idx >= 0 && next_idx < n {
            if arr[i as usize] < arr[prev_idx as usize]
                && arr[i as usize] < arr[next_idx as usize]
            {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
