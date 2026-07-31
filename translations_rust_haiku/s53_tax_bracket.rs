use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(bracket) = line.trim().parse::<i32>() {
            let tax = match bracket {
                4 => 800 + 400 + 200 + 100,
                3 => 400 + 200 + 100,
                2 => 200 + 100,
                1 => 100,
                _ => 0,
            };
            println!("{}", tax);
        }
    }
}
