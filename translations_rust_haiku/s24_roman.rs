use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(mut n) = line.trim().parse::<i32>() {
            let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
            let syms = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

            for i in 0..13 {
                while n >= vals[i] {
                    print!("{}", syms[i]);
                    n -= vals[i];
                }
            }
            println!();
        }
    }
}
