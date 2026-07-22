use std::io::{self, Read};

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

    let mut sum = 0.0;
    for _ in 0..n {
        let x: f64 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => break,
        };
        sum += x;
    }

    let avg = if n > 0 { sum / (n as f64) } else { 0.0 };
    println!("{}", avg);
}
