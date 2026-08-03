use std::io;

fn emit_string(bytes: &[u8], pos: &mut usize) {
    *pos += 1; // skip opening quote
    let mut buf = String::new();
    while *pos < bytes.len() && bytes[*pos] as char != '"' && buf.len() < 255 {
        buf.push(bytes[*pos] as char);
        *pos += 1;
    }
    if *pos < bytes.len() && bytes[*pos] as char == '"' {
        *pos += 1;
    }
    println!("STRING:{}", buf);
}

fn emit_number(bytes: &[u8], pos: &mut usize) {
    let mut buf = String::new();
    while *pos < bytes.len() {
        let c = bytes[*pos] as char;
        if c.is_ascii_digit() || c == '-' || c == '.' {
            buf.push(c);
            *pos += 1;
        } else {
            break;
        }
    }
    println!("NUMBER:{}", buf);
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line = line.trim_end_matches('\n').trim_end_matches('\r').to_string();

    let bytes = line.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        let c = bytes[pos] as char;
        if c == ' ' || c == '\t' {
            pos += 1;
        } else if c == '{' || c == '}' {
            println!("BRACE:{}", c);
            pos += 1;
        } else if c == '[' || c == ']' {
            println!("BRACKET:{}", c);
            pos += 1;
        } else if c == ':' {
            println!("COLON::");
            pos += 1;
        } else if c == ',' {
            println!("COMMA:,");
            pos += 1;
        } else if c == '"' {
            emit_string(&bytes, &mut pos);
        } else if c.is_ascii_digit() || c == '-' {
            emit_number(&bytes, &mut pos);
        } else if bytes[pos..].starts_with(b"true") {
            println!("BOOL:true");
            pos += 4;
        } else if bytes[pos..].starts_with(b"false") {
            println!("BOOL:false");
            pos += 5;
        } else if bytes[pos..].starts_with(b"null") {
            println!("NULL:null");
            pos += 4;
        } else {
            pos += 1;
        }
    }
}
