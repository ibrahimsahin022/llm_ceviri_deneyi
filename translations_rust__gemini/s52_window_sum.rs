use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => process::exit(1),
    };

    let k: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => process::exit(1),
    };

    if n < 0 || k < 0 {
        process::exit(1);
    }

    let mut arr = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let val: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => process::exit(1),
        };
        arr.push(val);
    }

    let mut start = n - k;
    if start < 0 {
        start = 0;
    }

    let mut sum: i64 = 0;
    for i in start..n {
        sum += arr[i as usize] as i64;
    }

    println!("{}", sum);
}
