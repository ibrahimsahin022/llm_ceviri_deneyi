use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 0..n {
                let t = a + b;
                a = b;
                b = t;
            }
            println!("{}", a);
        }
    }
}
