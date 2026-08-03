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

    if n < 2 || n > 1000 {
        return;
    }

    let mut arr = Vec::with_capacity(n as usize);
    for _ in 0..n {
        match tokens.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(val) => arr.push(val),
            None => return,
        }
    }

    let mut count = 0;
    for i in 1..(n as usize - 1) {
        if arr[i] < arr[i - 1] && arr[i] < arr[i + 1] {
            count += 1;
        }
    }

    println!("{}", count);
}
