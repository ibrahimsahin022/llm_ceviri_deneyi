use std::io::{self, Read};

fn main() {
    let mut chars: i32 = 0;
    let mut words: i32 = 0;
    let mut inword = false;

    for b in io::stdin().lock().bytes() {
        let c = match b {
            Ok(byte) => byte,
            Err(_) => break,
        };
        chars = chars.wrapping_add(1);
        if c == b' ' || c == b'\n' || c == b'\t' || c == b'\r' {
            inword = false;
        } else if !inword {
            words = words.wrapping_add(1);
            inword = true;
        }
    }

    println!("{} {}", words, chars);
}
