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

    let mut sum: i64 = 0;
    for _ in 0..n {
        if let Some(x) = tokens.next().and_then(|s| s.parse::<i64>().ok()) {
            sum = sum.wrapping_add(x);
        } else {
            break;
        }
    }

    println!("{}", sum);
}
