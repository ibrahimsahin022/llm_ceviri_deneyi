use std::io::{self, Read};

fn move_disks(n: i32, from: i32, via: i32, to: i32) {
    if n > 1 {
        move_disks(n - 1, from, to, via);
        println!("Move disk from pole {} to pole {}", from, to);
        move_disks(n - 1, via, from, to);
    } else {
        println!("Move disk from pole {} to pole {}", from, to);
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if n > 0 {
        move_disks(n, 1, 2, 3);
    }
}
