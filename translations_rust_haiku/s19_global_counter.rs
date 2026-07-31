use std::io::{self, BufRead};
use std::sync::Mutex;
use std::sync::OnceLock;

fn call_count() -> &'static Mutex<i32> {
    static COUNTER: OnceLock<Mutex<i32>> = OnceLock::new();
    COUNTER.get_or_init(|| Mutex::new(0))
}

fn next_id() -> i32 {
    let mut count = call_count().lock().unwrap();
    *count += 1;
    *count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            for _ in 0..n {
                println!("{}", next_id());
            }
        }
    }
}
