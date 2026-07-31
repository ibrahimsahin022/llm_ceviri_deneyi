use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut line)) = lines.next() {
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let mut j = i;
            while j < chars.len() && chars[j] == chars[i] {
                j += 1;
            }
            print!("{}{}", chars[i], j - i);
            i = j;
        }
        println!();
    }
}
