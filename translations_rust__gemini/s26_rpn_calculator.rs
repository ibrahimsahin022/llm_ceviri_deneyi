use std::io::{self, BufRead};

fn is_operator(tok: &str) -> bool {
    tok == "+" || tok == "-" || tok == "*" || tok == "/"
}

fn is_number(tok: &str) -> bool {
    let bytes = tok.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let mut i = 0;
    if bytes[0] == b'-' || bytes[0] == b'+' {
        i = 1;
    }
    if i >= bytes.len() {
        return false;
    }
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            return false;
        }
        i += 1;
    }
    true
}

fn apply_op(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "+" => a.wrapping_add(b),
        "-" => a.wrapping_sub(b),
        "*" => a.wrapping_mul(b),
        "/" => {
            if b == 0 {
                0
            } else {
                a.wrapping_div(b)
            }
        }
        _ => 0,
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }

    let mut stack = [0i64; 256];
    let mut sp = 0usize;

    let mut push = |v: i64| {
        if sp < 256 {
            stack[sp] = v;
            sp += 1;
        }
    };

    let mut pop = || -> i64 {
        if sp == 0 {
            return 0;
        }
        sp -= 1;
        stack[sp]
    };

    for tok in line.split_whitespace() {
        if is_number(tok) {
            let val = tok.parse::<i64>().unwrap_or(0);
            push(val);
        } else if is_operator(tok) {
            let b = pop();
            let a = pop();
            push(apply_op(tok, a, b));
        }
    }

    let result = pop();
    println!("{}", result);
}
