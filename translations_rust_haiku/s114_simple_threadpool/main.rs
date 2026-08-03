mod pool;

use std::io;
use std::thread;
use pool::TaskPoolSync;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let n_tasks: usize = parts[0].parse().unwrap();
    let n_workers: usize = parts[1].parse().unwrap();

    if n_tasks as i32 > 100000 || n_workers == 0 || n_workers > 64 {
        return;
    }

    let pool = TaskPoolSync::new(n_tasks);

    let mut handles = vec![];

    for _ in 0..n_workers {
        let pool_clone = pool.clone();
        let handle = thread::spawn(move || {
            while let Some(task_id) = pool_clone.next_task() {
                let v = ((task_id + 1) as i64) * ((task_id + 1) as i64);
                pool_clone.add_result(v);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", pool.total());
}
