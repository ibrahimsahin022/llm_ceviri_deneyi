mod queue;

use queue::Queue;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut q = Queue::new();

    for _ in 0..n {
        let op = it.next().unwrap();
        if op == "E" {
            let v: i32 = it.next().unwrap().parse().unwrap();
            if q.enqueue(v) {
                println!("OK");
            } else {
                println!("FULL");
            }
        } else {
            match q.dequeue() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            }
        }
    }

    println!("size={}", q.size());
}
