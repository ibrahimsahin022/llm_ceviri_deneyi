use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }

    let mut chars: i64 = 0;
    let mut words: i64 = 0;
    let mut inword = false;

    for &b in &input {
        chars += 1;
        if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
            inword = false;
        } else if !inword {
            words += 1;
            inword = true;
        }
    }

    println!("{} {}", words, chars);
}
