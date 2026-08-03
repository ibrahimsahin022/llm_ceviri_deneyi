use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    if n < 2 || n > 100 {
        return;
    }

    let mut arr = [0i32; 100];
    for k in 0..n {
        match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => arr[k] = val,
            None => return,
        }
    }

    let tmp: i32;
    let mut i: usize = 0;

    tmp = arr[i];
    i += 1;

    arr[i] = arr[1];
    i += 1;

    arr[1] = tmp;

    for k in 0..n {
        print!("{} ", arr[k]);
    }
    println!();
    println!("i={}", i);
}
