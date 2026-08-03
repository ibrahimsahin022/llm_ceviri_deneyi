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
    if n < 2 || n > 100 {
        return;
    }

    if integers.len() < 1 + n as usize {
        return;
    }

    let mut arr = integers[1..(n as usize + 1)].to_vec();
    let mut tmp: i32;
    let mut i = 0;

    // SWAP(arr[i++], arr[1], tmp) expands to:
    // tmp = arr[i++]; arr[i++] = arr[1]; arr[1] = tmp;
    tmp = arr[i];
    i += 1;
    arr[i] = arr[1];
    i += 1;
    arr[1] = tmp;

    for k in 0..n {
        print!("{} ", arr[k as usize]);
    }
    println!();
    println!("i={}", i);
}
