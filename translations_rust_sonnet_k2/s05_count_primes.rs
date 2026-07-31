use std::io::{self, Read};

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut d: i64 = 2;
    while d * d <= x as i64 {
        if x % (d as i32) == 0 {
            return false;
        }
        d += 1;
    }
    true
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

    let mut count = 0;
    for i in 2..=n {
        if is_prime(i) {
            count += 1;
        }
    }

    println!("{}", count);
}
