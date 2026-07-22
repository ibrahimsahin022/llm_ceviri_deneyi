use std::io::{self, Read};

// C'deki global 'static int call_count' dogrudan Rust'a 'static mut' olarak tasindi.
static mut CALL_COUNT: i32 = 0;

fn next_id() -> i32 {
    CALL_COUNT += 1;
    CALL_COUNT
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    for _ in 0..n {
        println!("{}", next_id());
    }
}
