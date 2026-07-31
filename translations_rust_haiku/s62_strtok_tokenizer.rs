use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    let mut idx = 0;
    for word in line.split(' ').filter(|w| !w.is_empty()) {
        println!("{}: {}", idx, word);
        idx += 1;
    }
}
