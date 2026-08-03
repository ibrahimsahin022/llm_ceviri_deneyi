use std::io::{self, BufRead};

const MUL_NO_OVERFLOW: u64 = 1u64 << 32;

fn reallocarray(nmemb: u64, size: u64) -> Option<u64> {
    if (nmemb >= MUL_NO_OVERFLOW || size >= MUL_NO_OVERFLOW)
        && nmemb > 0
        && u64::MAX / nmemb < size
    {
        return None;
    }
    Some(size * nmemb)
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut it = line.split_whitespace();
    let nmemb: u64 = it.next().unwrap().parse().unwrap();
    let size: u64 = it.next().unwrap().parse().unwrap();

    match reallocarray(nmemb, size) {
        None => println!("OVERFLOW"),
        Some(total) => println!("OK size={}", total),
    }
}
