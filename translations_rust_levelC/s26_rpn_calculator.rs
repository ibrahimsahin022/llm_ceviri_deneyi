use std::io::{self, Read};

fn apply_op(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0 {
                0
            } else {
                a / b
            }
        }
        _ => 0,
    }
}

fn is_operator(tok: &str) -> bool {
    matches!(tok, "+" | "-" | "*" | "/")
}

fn main() {
    let mut line = String::new();
    io::stdin().read_to_string(&mut line).unwrap();
    let first_line = line.lines().next().unwrap_or("");

    let mut stack: Vec<i64> = Vec::new();

    for tok in first_line.split_whitespace() {
        if is_operator(tok) {
            let b = stack.pop().unwrap_or(0);
            let a = stack.pop().unwrap_or(0);
            stack.push(apply_op(tok, a, b));
        } else if let Ok(num) = tok.parse::<i64>() {
            stack.push(num);
        }
    }

    let result = stack.pop().unwrap_or(0);
    println!("{}", result);
}
