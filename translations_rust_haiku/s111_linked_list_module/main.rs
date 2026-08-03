mod list;

use std::io::{self, BufRead};
use list::List;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let n: usize = first_line.trim().parse().unwrap();

    let mut l = List::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let v: i32 = parts.next().unwrap().parse().unwrap();

        if op == "P" {
            l.push_front(v);
        } else if op == "R" {
            if l.remove(v) {
                println!("REMOVED");
            } else {
                println!("NOTFOUND");
            }
        } else {
            if l.contains(v) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }

    println!("size={}", l.size());
}
