use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

struct SharedState {
    counter: i64,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n_threads, increments): (i32, i32) = {
        let line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            (parts[0].parse().unwrap_or(-1), parts[1].parse().unwrap_or(-1))
        } else {
            (-1, -1)
        }
    };

    if n_threads <= 0 || n_threads > 64 || increments < 0 {
        return;
    }

    let state = Arc::new(Mutex::new(SharedState { counter: 0 }));
    let mut handles = vec![];

    for _ in 0..n_threads {
        let state_clone = Arc::clone(&state);
        let increments_local = increments;
        let handle = thread::spawn(move || {
            for _ in 0..increments_local {
                let mut s = state_clone.lock().unwrap();
                s.counter += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().ok();
    }

    let final_state = state.lock().unwrap();
    println!("{}", final_state.counter);
}
