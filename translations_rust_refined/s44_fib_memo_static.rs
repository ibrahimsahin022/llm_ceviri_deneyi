use std::io::{self, Read};

static mut CACHE: [i64; 50] = [0; 50];
static mut HAVE: [bool; 50] = [false; 50];

fn fib_cached(n: i32) -> i64 {
    unsafe {
        if n < 2 {
            return n as i64;
        }
        if HAVE[n as usize] {
            return CACHE[n as usize];
        }
        let r = fib_cached(n - 1) + fib_cached(n - 2);
        CACHE[n as usize] = r;
        HAVE[n as usize] = true;
        r
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    println!("{}", fib_cached(n));
}
