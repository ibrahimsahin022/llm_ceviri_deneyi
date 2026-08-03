use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n < 0 || n > 1000 {
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
    let k: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let mut start = n - k;
    if start < 0 {
        start = 0;
    }
    for i in start..n {
        print!("{} ", arr[i as usize]);
    }
    println!();
}
