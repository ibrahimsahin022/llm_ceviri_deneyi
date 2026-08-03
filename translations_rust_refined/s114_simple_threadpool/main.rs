mod pool;

use pool::TaskPool;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_tasks: i32 = it.next().unwrap().parse().unwrap();
    let n_workers: i32 = it.next().unwrap().parse().unwrap();

    if n_tasks < 0 || n_tasks > 100000 || n_workers <= 0 || n_workers > 64 {
        return;
    }

    let pool = Arc::new(TaskPool::new(n_tasks));
    let mut handles = Vec::new();

    for _ in 0..n_workers {
        let p = pool.clone();
        handles.push(thread::spawn(move || {
            while let Some(task_id) = p.next_task() {
                let v = (task_id as i64 + 1) * (task_id as i64 + 1);
                p.add_result(v);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", pool.total());
}
