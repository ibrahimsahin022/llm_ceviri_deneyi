use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;

struct TaskPoolInner {
    next_task: i32,
    n_tasks: i32,
    total: i64,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut words = input.split_whitespace();
    let n_tasks: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };
    let n_workers: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };

    if n_tasks < 0 || n_tasks > 100000 || n_workers <= 0 || n_workers > 64 {
        std::process::exit(1);
    }

    let pool = Arc::new(Mutex::new(TaskPoolInner {
        next_task: 0,
        n_tasks,
        total: 0,
    }));

    let mut handles = Vec::with_capacity(n_workers as usize);

    for _ in 0..n_workers {
        let pool_clone = Arc::clone(&pool);
        let handle = thread::spawn(move || loop {
            let task_id = {
                let mut guard = pool_clone.lock().unwrap();
                if guard.next_task < guard.n_tasks {
                    let id = guard.next_task;
                    guard.next_task += 1;
                    Some(id)
                } else {
                    None
                }
            };

            match task_id {
                Some(id) => {
                    let v = (id as i64 + 1) * (id as i64 + 1);
                    let mut guard = pool_clone.lock().unwrap();
                    guard.total += v;
                }
                None => break,
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let total = pool.lock().unwrap().total;
    println!("{}", total);
}
