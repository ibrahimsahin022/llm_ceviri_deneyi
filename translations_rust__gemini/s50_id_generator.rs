use std::io::{self, Read};
use std::sync::atomic::{AtomicI32, Ordering};

const BASLANGIC_ID: i32 = 1000;

fn next_id() -> i32 {
    static COUNTER: AtomicI32 = AtomicI32::new(BASLANGIC_ID);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut words = input.split_whitespace();
    if let Some(word) = words.next() {
        if let Ok(n) = word.parse::<i32>() {
            for _ in 0..n {
                println!("{}", next_id());
            }
            return;
        }
    }
    std::process::exit(1);
}
