use std::io::{self, BufRead};

const LONG_MAX: i32 = 2147483647;
const LONG_MIN: i32 = -2147483648;

fn safe_add_clamped(a: i32, b: i32) -> i32 {
    let sum = (a as f64) + (b as f64);
    if sum > (LONG_MAX as f64) {
        return LONG_MAX;
    }
    if sum < (LONG_MIN as f64) {
        return LONG_MIN;
    }
    a + b
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                println!("{}", safe_add_clamped(a, b));
            }
        }
    }
}
