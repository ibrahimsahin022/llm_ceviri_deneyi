use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let cap: usize = it.next().unwrap().parse().unwrap();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut idx: usize = 0;
    for _ in 0..n {
        let op: i32 = it.next().unwrap().parse().unwrap();
        if op == 1 {
            idx = (idx + 1) % cap;
        } else {
            idx = (idx + cap - 1) % cap;
        }
        println!("{}", idx);
    }
}
