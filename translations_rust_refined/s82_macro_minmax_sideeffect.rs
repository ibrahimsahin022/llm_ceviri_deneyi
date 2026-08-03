use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut x: i32 = it.next().unwrap().parse().unwrap();
    let y: i32 = it.next().unwrap().parse().unwrap();

    let cond1 = { let t = x; x += 1; t };
    let m = if cond1 < y {
        let t = x; x += 1; t
    } else {
        y
    };

    let cond2 = { let t = x; x += 1; t };
    let mm = if cond2 > y {
        let t = x; x += 1; t
    } else {
        y
    };

    println!("{} {} {}", m, mm, x);
}
