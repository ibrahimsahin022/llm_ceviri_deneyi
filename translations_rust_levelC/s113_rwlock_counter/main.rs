mod rwstate;

use rwstate::RwState;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_writers: i32 = it.next().unwrap().parse().unwrap();
    let increments: i32 = it.next().unwrap().parse().unwrap();

    if n_writers <= 0 || n_writers > 64 || increments < 0 {
        return;
    }

    let state = Arc::new(RwState::new());
    let mut handles = Vec::new();

    for _ in 0..n_writers {
        let s = state.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                s.increment();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", state.read());
}
