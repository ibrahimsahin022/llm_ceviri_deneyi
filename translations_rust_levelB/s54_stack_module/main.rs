mod stack;

use std::io::{self, Read};
use stack::Stack;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let mut s = Stack::new();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let cmd = match tokens.next() {
            Some(c) => c,
            None => break,
        };
        match cmd {
            "PUSH" => {
                let v: i32 = tokens.next().unwrap().parse().unwrap();
                if s.push(v) {
                    println!("OK");
                } else {
                    println!("FULL");
                }
            }
            "POP" => match s.pop() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            },
            "PEEK" => match s.peek() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            },
            "SIZE" => println!("{}", s.size()),
            _ => {}
        }
    }
}
