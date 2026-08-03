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

    let n = integers[0];
    if n < 0 || n > 1000 {
        return;
    }

    if integers.len() < (n as usize) + 2 {
        return;
    }

    let arr = &integers[1..(n as usize + 1)];
    let k = integers[n as usize + 1];

    let mut start = n - k;
    if start < 0 {
        start = 0;
    }

    for i in (start as usize)..arr.len() {
        print!("{} ", arr[i]);
    }
    println!();
}
