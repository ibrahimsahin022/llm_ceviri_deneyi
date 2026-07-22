use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|t| t.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut limit = n;
    if limit > 1000 {
        limit = 1000;
    }

    let mut a = [0i32; 1000];
    let mut count = 0;

    if limit > 0 {
        for i in 0..(limit as usize) {
            if let Some(val) = tokens.next().and_then(|t| t.parse().ok()) {
                a[i] = val;
                count += 1;
            } else {
                break;
            }
        }
    }

    a[..count].sort();

    for i in 0..count {
        print!("{}", a[i]);
        if i + 1 < count {
            print!(" ");
        }
    }
    println!();
}
