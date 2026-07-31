use std::io::{self, Read};

fn next_id(call_count: &mut i32) -> i32 {
    *call_count += 1;
    *call_count
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

    let mut call_count: i32 = 0;
    for _ in 0..n {
        println!("{}", next_id(&mut call_count));
    }
}
