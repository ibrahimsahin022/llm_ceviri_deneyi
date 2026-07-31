use std::io::{self, Read};

fn fib_cached(n: i32, cache: &mut [i64; 50], have: &mut [bool; 50]) -> i64 {
    if n < 2 {
        return n as i64;
    }
    if have[n as usize] {
        return cache[n as usize];
    }
    let r = fib_cached(n - 1, cache, have) + fib_cached(n - 2, cache, have);
    cache[n as usize] = r;
    have[n as usize] = true;
    r
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

    let mut cache = [0i64; 50];
    let mut have = [false; 50];

    println!("{}", fib_cached(n, &mut cache, &mut have));
}
