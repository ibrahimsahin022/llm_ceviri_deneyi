use std::io;

fn skip_spaces(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() {
        let c = bytes[*pos] as char;
        if c == ' ' || c == '\t' {
            *pos += 1;
        } else {
            break;
        }
    }
}

fn match_word(word: &str, bytes: &[u8], pos: &mut usize) -> bool {
    let word_bytes = word.as_bytes();
    if *pos < bytes.len() && bytes[*pos..].starts_with(word_bytes) {
        *pos += word.len();
        true
    } else {
        false
    }
}

fn parse_factor(bytes: &[u8], pos: &mut usize) -> bool {
    skip_spaces(bytes, pos);
    if *pos < bytes.len() {
        let c = bytes[*pos] as char;
        if c == '(' {
            *pos += 1;
            let v = parse_or(bytes, pos);
            skip_spaces(bytes, pos);
            if *pos < bytes.len() && bytes[*pos] as char == ')' {
                *pos += 1;
            }
            return v;
        }
        if match_word("NOT", bytes, pos) {
            let v = parse_factor(bytes, pos);
            return !v;
        }
        if c == '0' {
            *pos += 1;
            return false;
        }
        if c == '1' {
            *pos += 1;
            return true;
        }
    }
    false
}

fn parse_and(bytes: &[u8], pos: &mut usize) -> bool {
    let mut value = parse_factor(bytes, pos);
    loop {
        skip_spaces(bytes, pos);
        if match_word("AND", bytes, pos) {
            let rhs = parse_factor(bytes, pos);
            value = value && rhs;
        } else {
            break;
        }
    }
    value
}

fn parse_or(bytes: &[u8], pos: &mut usize) -> bool {
    let mut value = parse_and(bytes, pos);
    loop {
        skip_spaces(bytes, pos);
        if match_word("OR", bytes, pos) {
            let rhs = parse_and(bytes, pos);
            value = value || rhs;
        } else {
            break;
        }
    }
    value
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let bytes = input.as_bytes();
    let mut pos = 0;
    let result = parse_or(bytes, &mut pos);
    println!("{}", if result { 1 } else { 0 });
}
