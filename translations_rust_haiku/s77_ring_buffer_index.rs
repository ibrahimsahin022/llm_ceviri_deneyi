use std::io;

fn main() {
    let mut input = String::new();
    let mut integers = Vec::new();

    while io::stdin().read_line(&mut input).is_ok() && !input.is_empty() {
        for word in input.trim().split_whitespace() {
            if let Ok(num) = word.parse::<i32>() {
                integers.push(num);
            }
        }
        input.clear();
    }

    if integers.len() < 2 {
        return;
    }

    let cap = integers[0];
    let n = integers[1];
    if cap <= 0 {
        return;
    }

    if integers.len() < 2 + n as usize {
        return;
    }

    let mut idx = 0;
    for t in 0..n as usize {
        let op = integers[2 + t];
        if op == 1 {
            idx = (idx + 1) % cap;
        } else {
            idx = (idx - 1 + cap) % cap;
        }
        println!("{}", idx);
    }
}
