use std::io::{self, Read};

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

    let mut sum: i64 = 0;
    for _ in 0..n {
        let x: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        sum += x;
    }

    println!("{}", sum);
}
