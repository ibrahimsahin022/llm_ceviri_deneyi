mod list;

use list::List;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut l = List::new();

    for _ in 0..n {
        let op = it.next().unwrap();
        let v: i32 = it.next().unwrap().parse().unwrap();
        match op {
            "P" => l.push_front(v),
            "R" => println!("{}", if l.remove(v) { "REMOVED" } else { "NOTFOUND" }),
            _ => println!("{}", if l.contains(v) { "YES" } else { "NO" }),
        }
    }

    println!("size={}", l.size());
}
