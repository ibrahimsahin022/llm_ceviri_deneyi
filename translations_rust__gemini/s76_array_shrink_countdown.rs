use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut iter = input.split_whitespace();

    let n: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    if n < 0 || n > 1000 {
        return;
    }

    let mut arr = [0i32; 1000];
    for i in 0..n as usize {
        arr[i] = match iter.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
    }

    let k: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut start = n - k;
    if start < 0 {
        start = 0;
    }

    for i in start..n {
        print!("{} ", arr[i as usize]);
    }
    println!();
}
