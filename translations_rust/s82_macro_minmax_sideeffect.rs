use std::io::{self, Read};

fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}
fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut x: i32 = it.next().unwrap().parse().unwrap();
    let y: i32 = it.next().unwrap().parse().unwrap();
    let m = min_i32(x, y);
    x += 1;
    let mm = max_i32(x, y);
    x += 1;
    println!("{} {} {}", m, mm, x);
}
