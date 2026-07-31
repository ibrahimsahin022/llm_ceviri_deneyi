use std::io::{self, BufRead};

fn hanoi_move(n: i32, from: i32, via: i32, to: i32) {
    if n > 1 {
        hanoi_move(n - 1, from, to, via);
        println!("Move disk from pole {} to pole {}", from, to);
        hanoi_move(n - 1, via, from, to);
    } else {
        println!("Move disk from pole {} to pole {}", from, to);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            if n > 0 {
                hanoi_move(n, 1, 2, 3);
            }
        }
    }
}
