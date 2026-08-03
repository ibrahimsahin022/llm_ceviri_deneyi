use std::io::{self, Read};

fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, ()> {
    if denominator == 0 {
        return Err(());
    }
    Ok(numerator / denominator)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    if n > 1000 {
        return;
    }

    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        values.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    let mut processed = 0;
    for &v in &values {
        match safe_divide(100, v) {
            Ok(result) => {
                println!("{}", result);
                processed += 1;
            }
            Err(()) => {
                println!("ERROR");
            }
        }
    }

    println!("processed={}", processed);
}
