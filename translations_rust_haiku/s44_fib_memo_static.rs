use std::io::{self, BufRead};
use std::sync::Mutex;
use std::sync::OnceLock;

struct FibCache {
    cache: Vec<i64>,
    have: Vec<i32>,
}

fn get_cache() -> &'static Mutex<FibCache> {
    static CACHE: OnceLock<Mutex<FibCache>> = OnceLock::new();
    CACHE.get_or_init(|| {
        Mutex::new(FibCache {
            cache: vec![0; 50],
            have: vec![0; 50],
        })
    })
}

fn fib_cached(n: usize) -> i64 {
    if n < 2 {
        return n as i64;
    }

    let mut cache_guard = get_cache().lock().unwrap();
    if cache_guard.have[n] != 0 {
        return cache_guard.cache[n];
    }
    drop(cache_guard);

    let r = fib_cached(n - 1) + fib_cached(n - 2);

    let mut cache_guard = get_cache().lock().unwrap();
    cache_guard.cache[n] = r;
    cache_guard.have[n] = 1;
    r
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<usize>() {
            println!("{}", fib_cached(n));
        }
    }
}
