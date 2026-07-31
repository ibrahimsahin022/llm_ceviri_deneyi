use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut arr: Vec<i32> = Vec::new();
    for _ in 0..n {
        let v: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        arr.push(v);
    }

    arr.sort();

    let mut out = String::new();
    for i in 0..arr.len() {
        out.push_str(&arr[i].to_string());
        if i + 1 < arr.len() {
            out.push(' ');
        } else {
            out.push('\n');
        }
    }
    if n == 0 {
        out.push('\n');
    }
    print!("{}", out);
}
