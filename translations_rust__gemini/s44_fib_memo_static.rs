use std::io::{self, Read};

fn fib_cached(n: i32) -> i64 {
    static mut CACHE: [i64; 50] = [0; 50];
    static mut HAVE: [i32; 50] = [0; 50];

    if n < 2 {
        return n as i64;
    }

    let idx = n as usize;
    unsafe {
        if HAVE[idx] != 0 {
            return CACHE[idx];
        }
        let r = fib_cached(n - 1) + fib_cached(n - 2);
        CACHE[idx] = r;
        HAVE[idx] = 1;
        r
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        let mut iter = input.split_whitespace();
        if let Some(token) = iter.next() {
            if let Ok(n) = token.parse::<i32>() {
                println!("{}", fib_cached(n));
            }
        }
    }
}
