use std::io::{self, Read};

fn move_disk(n: i32, from: i32, via: i32, to: i32) {
    if n > 1 {
        move_disk(n - 1, from, to, via);
        println!("Move disk from pole {} to pole {}", from, to);
        move_disk(n - 1, via, from, to);
    } else {
        println!("Move disk from pole {} to pole {}", from, to);
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        if let Some(word) = input.split_whitespace().next() {
            if let Ok(n) = word.parse::<i32>() {
                if n > 0 {
                    move_disk(n, 1, 2, 3);
                }
            }
        }
    }
}
