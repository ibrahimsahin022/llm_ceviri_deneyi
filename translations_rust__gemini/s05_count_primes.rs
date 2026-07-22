use std::io::{self, Read};

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut d = 2i32;
    while (d as i64) * (d as i64) <= (x as i64) {
        if x % d == 0 {
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
    let mut tokens = input.split_whitespace();
    if let Some(token) = tokens.next() {
        if let Ok(n) = token.parse::<i32>() {
            let mut count = 0;
            for i in 2..=n {
                if is_prime(i) {
                    count += 1;
                }
            }
            println!("{}", count);
        }
    }
}
