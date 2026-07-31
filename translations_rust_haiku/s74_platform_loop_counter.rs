use std::io;

fn safe_mul_clamped(a: i32, b: i32) -> i32 {
    let product = (a as f64) * (b as f64);
    const LONG_MAX: f64 = 2147483647.0;
    const LONG_MIN: f64 = -2147483648.0;

    if product > LONG_MAX {
        2147483647
    } else if product < LONG_MIN {
        -2147483648
    } else {
        a * b
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return;
    }

    let a: i32 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let b: i32 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    println!("{}", safe_mul_clamped(a, b));
}
