use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    let mut num_str = String::new();
    if let Some(&c) = chars.peek() {
        if c == '+' || c == '-' {
            num_str.push(c);
            chars.next();
        }
    }

    let mut has_digit = false;
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            has_digit = true;
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }

    if !has_digit {
        return;
    }

    if let Ok(code) = num_str.parse::<i32>() {
        let tier = code / 100;
        let score = match tier {
            5 => 8 + 4 + 2 + 1,
            4 => 4 + 2 + 1,
            3 => 2 + 1,
            2 => 1,
            _ => 0,
        };
        println!("{}", score);
    }
}
