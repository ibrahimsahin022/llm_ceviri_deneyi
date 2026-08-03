use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let cap: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if cap <= 0 {
        return;
    }
    let mut idx: i32 = 0;
    for _ in 0..n {
        let op: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        if op == 1 {
            idx = (idx + 1) % cap;
        } else {
            idx = (idx - 1 + cap) % cap;
        }
        println!("{}", idx);
    }
}
