use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut n: u64 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut steps: u64 = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.wrapping_mul(3).wrapping_add(1);
        }
        steps += 1;
    }

    println!("{}", steps);
}
