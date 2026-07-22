use std::io::{self, Read};
use std::sync::atomic::{AtomicI32, Ordering};

static CALL_COUNT: AtomicI32 = AtomicI32::new(0);

fn next_id() -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut words = input.split_whitespace();
    if let Some(word) = words.next() {
        if let Ok(n) = word.parse::<i32>() {
            for _ in 0..n {
                println!("{}", next_id());
            }
        }
    }
}
