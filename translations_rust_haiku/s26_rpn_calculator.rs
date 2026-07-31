use std::io::{self, BufRead};

fn is_operator(tok: &str) -> bool {
    tok == "+" || tok == "-" || tok == "*" || tok == "/"
}

fn is_number(tok: &str) -> bool {
    let mut i = 0;
    let chars: Vec<char> = tok.chars().collect();
    if chars.is_empty() {
        return false;
    }
    if chars[0] == '-' || chars[0] == '+' {
        i = 1;
    }
    if i >= chars.len() {
        return false;
    }
    for j in i..chars.len() {
        if chars[j] < '0' || chars[j] > '9' {
            return false;
        }
    }
    true
}

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

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut stack: Vec<i64> = Vec::new();

        for tok in line.split_whitespace() {
            if is_number(tok) {
                if let Ok(v) = tok.parse::<i64>() {
                    stack.push(v);
                }
            } else if is_operator(tok) {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(apply_op(tok, a, b));
                }
            }
        }

        let result = stack.last().copied().unwrap_or(0);
        println!("{}", result);
    }
}
