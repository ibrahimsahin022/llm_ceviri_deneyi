use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut tokens = input.split_whitespace();
    let n_threads: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let increments: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    if n_threads <= 0 || n_threads > 64 || increments < 0 {
        std::process::exit(1);
    }

    let state = Arc::new(Mutex::new(0i64));

    let mut handles = Vec::with_capacity(n_threads as usize);
    for _ in 0..n_threads {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                let mut guard = state_clone.lock().unwrap();
                *guard += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let final_val = *state.lock().unwrap();
    println!("{}", final_val);
}
