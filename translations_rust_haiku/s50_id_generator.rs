use std::io::{self, BufRead};
use std::sync::Mutex;
use std::sync::OnceLock;

const BASLANGIC_ID: i32 = 1000;

fn counter() -> &'static Mutex<i32> {
    static COUNTER: OnceLock<Mutex<i32>> = OnceLock::new();
    COUNTER.get_or_init(|| Mutex::new(BASLANGIC_ID))
}

fn next_id() -> i32 {
    let mut c = counter().lock().unwrap();
    let id = *c;
    *c += 1;
    id
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
