use std::io::{self, Read};

fn sum_slice(args: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &val in args {
        total += val as i64;
    }
    total
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if n < 0 || n > 20 {
        return;
    }

    let mut values = Vec::new();
    for _ in 0..n {
        match tokens.next().and_then(|s| s.parse().ok()) {
            Some(v) => values.push(v),
            None => return,
        }
    }

    let total: i64 = match n {
        0 => sum_slice(&[]),
        1 => sum_slice(&values[..1]),
        2 => sum_slice(&values[..2]),
        3 => sum_slice(&values[..3]),
        _ => {
            let mut tot: i64 = 0;
            for i in 0..n as usize {
                tot += values[i] as i64;
            }
            tot
        }
    };

    println!("{}", total);
}
