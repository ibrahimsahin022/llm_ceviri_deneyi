use std::io::{self, Read};

fn is_number(tok: &[u8]) -> bool {
    let mut i = 0usize;
    if !tok.is_empty() && (tok[0] == b'-' || tok[0] == b'+') {
        i = 1;
    }
    if i >= tok.len() {
        return false;
    }
    for &b in &tok[i..] {
        if !(b'0'..=b'9').contains(&b) {
            return false;
        }
    }
    true
}

fn is_operator(tok: &[u8]) -> bool {
    tok == b"+" || tok == b"-" || tok == b"*" || tok == b"/"
}

fn apply_op(op: &[u8], a: i64, b: i64) -> i64 {
    if op == b"+" {
        a + b
    } else if op == b"-" {
        a - b
    } else if op == b"*" {
        a * b
    } else if op == b"/" {
        if b == 0 {
            0
        } else {
            a / b
        }
    } else {
        0
    }
}

fn atol_bytes(tok: &[u8]) -> i64 {
    std::str::from_utf8(tok)
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    if input.is_empty() {
        return;
    }
    let mut end = 0usize;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    let line = &input[..end];

    // tokenize on runs of ' ', '\t', '\n' (like strtok(line, " \t\n"))
    let mut tokens: Vec<&[u8]> = Vec::new();
    let mut i = 0usize;
    let n = line.len();
    while i < n {
        while i < n && (line[i] == b' ' || line[i] == b'\t' || line[i] == b'\n') {
            i += 1;
        }
        if i >= n {
            break;
        }
        let start = i;
        while i < n && line[i] != b' ' && line[i] != b'\t' && line[i] != b'\n' {
            i += 1;
        }
        tokens.push(&line[start..i]);
    }

    let mut stack: Vec<i64> = Vec::with_capacity(256);
    const STACK_CAP: usize = 256;

    for tok in tokens {
        if is_number(tok) {
            if stack.len() < STACK_CAP {
                stack.push(atol_bytes(tok));
            }
        } else if is_operator(tok) {
            let b = stack.pop().unwrap_or(0);
            let a = stack.pop().unwrap_or(0);
            let r = apply_op(tok, a, b);
            if stack.len() < STACK_CAP {
                stack.push(r);
            }
        }
    }

    let result = stack.pop().unwrap_or(0);
    println!("{}", result);
}
