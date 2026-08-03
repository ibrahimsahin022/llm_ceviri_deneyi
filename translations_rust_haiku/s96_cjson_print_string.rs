use std::io::{self, BufRead};

fn print_string_ptr(input: &[u8]) -> String {
    let mut result = String::from("\"");

    for &byte in input {
        if byte > 31 && byte != b'"' && byte != b'\\' {
            result.push(byte as char);
        } else {
            result.push('\\');
            match byte {
                b'\\' => result.push('\\'),
                b'"' => result.push('"'),
                b'\x08' => result.push('b'),
                b'\x0c' => result.push('f'),
                b'\n' => result.push('n'),
                b'\r' => result.push('r'),
                b'\t' => result.push('t'),
                _ => {
                    result.push('u');
                    result.push_str(&format!("{:04x}", byte));
                }
            }
        }
    }

    result.push('"');
    result
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    if stdin.read_line(&mut line).is_err() {
        return;
    }

    let trimmed = line.trim_end();
    let output = print_string_ptr(trimmed.as_bytes());
    println!("{}", output);
}
