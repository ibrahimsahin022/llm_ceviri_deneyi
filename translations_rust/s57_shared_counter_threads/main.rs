use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n_threads = it.next().unwrap();
    let increments = it.next().unwrap();
    if n_threads <= 0 || n_threads > 64 || increments < 0 {
        return;
    }

    let counter = Arc::new(Mutex::new(0i64));
    let mut handles = Vec::new();

    for _ in 0..n_threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                let mut c = counter.lock().unwrap();
                *c += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
