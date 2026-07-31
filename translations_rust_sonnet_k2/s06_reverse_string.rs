use std::io::{self, Read, Write};

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
    let mut line = input[..end].to_vec();

    while let Some(&last) = line.last() {
        if last == b'\n' || last == b'\r' {
            line.pop();
        } else {
            break;
        }
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut result: Vec<u8> = line.iter().rev().cloned().collect();
    result.push(b'\n');
    let _ = out.write_all(&result);
}
