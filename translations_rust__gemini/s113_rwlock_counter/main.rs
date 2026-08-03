use std::io::{self, Read};
use std::process;
use std::sync::{Arc, RwLock};
use std::thread;

struct RwState {
    counter: RwLock<i64>,
}

impl RwState {
    fn new() -> Self {
        RwState {
            counter: RwLock::new(0),
        }
    }

    fn increment(&self) {
        let mut guard = self.counter.write().unwrap();
        *guard += 1;
    }

    fn read(&self) -> i64 {
        let guard = self.counter.read().unwrap();
        *guard
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut tokens = input.split_whitespace();

    let n_writers: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    let increments: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    if n_writers <= 0 || n_writers > 64 || increments < 0 {
        process::exit(1);
    }

    let state = Arc::new(RwState::new());
    let mut handles = Vec::with_capacity(n_writers as usize);

    for _ in 0..n_writers {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                state_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    println!("{}", state.read());
}
