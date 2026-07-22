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
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n > 0 {
        move_disks(n, 1, 2, 3);
    }
}
