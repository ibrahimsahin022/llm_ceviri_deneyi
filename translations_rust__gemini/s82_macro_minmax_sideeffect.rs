use std::io::{self, Read};

macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}

fn post_inc(x: &mut i32) -> i32 {
    let old = *x;
    *x += 1;
    old
}

#[allow(non_snake_case)]
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut iter = input.split_whitespace();
    let mut x: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let y: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let m = min!(post_inc(&mut x), y);
    let M = max!(post_inc(&mut x), y);
    println!("{} {} {}", m, M, x);
}
