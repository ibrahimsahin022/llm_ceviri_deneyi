use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) if (0..=1000).contains(&val) => val,
        _ => return,
    };

    let mut values = Vec::with_capacity(n as usize);
    for _ in 0..n {
        match tokens.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(val) => values.push(val),
            None => return,
        }
    }

    let mut processed = 0;
    for &val in &values {
        if val == 0 {
            println!("ERROR");
        } else {
            let result = 100 / val;
            println!("{}", result);
            processed += 1;
        }
    }

    println!("processed={}", processed);
}
