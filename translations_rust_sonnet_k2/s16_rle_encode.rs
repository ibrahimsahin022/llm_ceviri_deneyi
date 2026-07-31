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

    let len = line.len();
    let mut i = 0usize;
    let mut out = String::new();
    while i < len {
        let mut j = i;
        while j < len && line[j] == line[i] {
            j += 1;
        }
        out.push_str(&format!("{}{}", line[i] as char, j - i));
        i = j;
    }
    out.push('\n');

    let stdout = io::stdout();
    let mut o = stdout.lock();
    let _ = o.write_all(out.as_bytes());
}
