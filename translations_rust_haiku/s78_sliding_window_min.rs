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

    if integers.is_empty() {
        return;
    }

    let n = integers[0];
    if n < 2 || n > 1000 {
        return;
    }

    if integers.len() < 1 + n as usize {
        return;
    }

    let arr = &integers[1..(n as usize + 1)];
    let mut count = 0;
    for i in 0..n {
        let prev_idx = i - 1;
        let next_idx = i + 1;
        if prev_idx >= 0 && next_idx < n {
            if arr[i as usize] < arr[prev_idx as usize] && arr[i as usize] < arr[next_idx as usize] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
