use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let c = chars[i];
        if c == ' ' || c == '\t' {
            i += 1;
        } else if c == '{' || c == '}' {
            println!("BRACE:{}", c);
            i += 1;
        } else if c == '[' || c == ']' {
            println!("BRACKET:{}", c);
            i += 1;
        } else if c == ':' {
            println!("COLON::");
            i += 1;
        } else if c == ',' {
            println!("COMMA:,");
            i += 1;
        } else if c == '"' {
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != '"' {
                s.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
            println!("STRING:{}", s);
        } else if c.is_ascii_digit() || c == '-' {
            let mut s = String::new();
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '-' || chars[i] == '.') {
                s.push(chars[i]);
                i += 1;
            }
            println!("NUMBER:{}", s);
        } else if chars[i..].iter().collect::<String>().starts_with("true") {
            println!("BOOL:true");
            i += 4;
        } else if chars[i..].iter().collect::<String>().starts_with("false") {
            println!("BOOL:false");
            i += 5;
        } else if chars[i..].iter().collect::<String>().starts_with("null") {
            println!("NULL:null");
            i += 4;
        } else {
            i += 1;
        }
    }
}
