mod rwstate;

use std::io;
use std::thread;
use rwstate::RwState;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let n_writers: usize = parts[0].parse().unwrap();
    let increments: i32 = parts[1].parse().unwrap();

    if n_writers <= 0 || n_writers > 64 || increments < 0 {
        return;
    }

    let state = RwState::new();

    let mut handles = vec![];

    for _ in 0..n_writers {
        let state_clone = state.clone();
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                state_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", state.read());
}
