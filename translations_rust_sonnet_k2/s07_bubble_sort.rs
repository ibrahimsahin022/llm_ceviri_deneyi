use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let mut n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = [0i32; 1000];
    if n > 1000 {
        n = 1000;
    }
    for i in 0..n {
        let v: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        a[i as usize] = v;
    }

    for i in 0..(n - 1) {
        for j in 0..(n - 1 - i) {
            let j = j as usize;
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }

    for i in 0..n {
        print!("{}", a[i as usize]);
        if i < n - 1 {
            print!(" ");
        }
    }
    println!();
}
