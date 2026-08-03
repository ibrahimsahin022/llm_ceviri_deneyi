use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let v: i64 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let threshold: i64 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let step: i64 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    if step <= 0 {
        return;
    }

    let mut v = v;
    let mut attempts: i32 = 0;

    loop {
        if v >= threshold {
            break;
        }
        v += step;
        attempts += 1;
        if attempts > 1000000 {
            break;
        }
    }

    println!("attempts={} final={}", attempts, v);
}
