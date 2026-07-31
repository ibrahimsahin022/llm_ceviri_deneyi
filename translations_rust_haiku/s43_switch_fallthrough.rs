use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(level) = line.trim().parse::<i32>() {
            let mut bonus = 0;
            match level {
                4 => {
                    bonus += 8;
                    bonus += 4;
                    bonus += 2;
                    bonus += 1;
                }
                3 => {
                    bonus += 4;
                    bonus += 2;
                    bonus += 1;
                }
                2 => {
                    bonus += 2;
                    bonus += 1;
                }
                1 => {
                    bonus += 1;
                }
                _ => {
                    bonus = 0;
                }
            }
            println!("{}", bonus);
        }
    }
}
