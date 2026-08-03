use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut list: Vec<i32> = Vec::new();

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };
        let v: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };

        let op_char = op.chars().next().unwrap_or('\0');
        if op_char == 'P' {
            list.insert(0, v);
        } else if op_char == 'R' {
            if let Some(pos) = list.iter().position(|&x| x == v) {
                list.remove(pos);
                println!("REMOVED");
            } else {
                println!("NOTFOUND");
            }
        } else {
            if list.contains(&v) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }

    println!("size={}", list.len());
}
