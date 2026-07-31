use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n_threads: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let increments: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if n_threads <= 0 || n_threads > 64 || increments < 0 {
        return;
    }

    let counter = Arc::new(Mutex::new(0i64));
    let mut handles = Vec::with_capacity(n_threads as usize);

    for _ in 0..n_threads {
        let counter = Arc::clone(&counter);
        let inc = increments;
        let handle = thread::spawn(move || {
            for _ in 0..inc {
                let mut c = counter.lock().unwrap();
                *c += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let final_value = *counter.lock().unwrap();
    println!("{}", final_value);
}
