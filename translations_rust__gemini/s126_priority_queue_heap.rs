use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next() {
        Some(s) => match s.parse() {
            Ok(val) => val,
            Err(_) => return,
        },
        None => return,
    };

    let mut heap = BinaryHeap::new();

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };

        if op.starts_with('I') {
            let v: i32 = match tokens.next() {
                Some(s) => match s.parse() {
                    Ok(val) => val,
                    Err(_) => return,
                },
                None => return,
            };
            heap.push(Reverse(v));
        } else {
            if let Some(Reverse(out)) = heap.pop() {
                println!("{}", out);
            } else {
                println!("EMPTY");
            }
        }
    }

    println!("size={}", heap.len());
}
