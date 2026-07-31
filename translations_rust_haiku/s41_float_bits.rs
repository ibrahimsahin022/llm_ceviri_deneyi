use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(x) = line.trim().parse::<f32>() {
            println!("{}", x.to_bits());
        }
    }
}
