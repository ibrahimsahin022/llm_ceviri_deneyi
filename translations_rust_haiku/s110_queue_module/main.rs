mod queue;

use std::io::{self, BufRead};
use queue::Queue;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.trim().parse().unwrap();

    let mut q = Queue::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();

        if op == "E" {
            let v: i32 = parts.next().unwrap().parse().unwrap();
            if q.enqueue(v) {
                println!("OK");
            } else {
                println!("FULL");
            }
        } else {
            if let Some(out) = q.dequeue() {
                println!("{}", out);
            } else {
                println!("EMPTY");
            }
        }
    }

    println!("size={}", q.size());
}
