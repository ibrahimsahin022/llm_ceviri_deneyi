use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut v: i64 = it.next().unwrap().parse().unwrap();
    let threshold: i64 = it.next().unwrap().parse().unwrap();
    let step: i64 = it.next().unwrap().parse().unwrap();

    if step <= 0 {
        return;
    }

    let mut attempts = 0i32;

    loop {
        if v >= threshold {
            break;
        }
        v += step;
        attempts += 1;
        if attempts > 1_000_000 {
            break;
        }
    }

    println!("attempts={} final={}", attempts, v);
}
