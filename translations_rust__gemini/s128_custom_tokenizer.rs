use std::io::{self, Read};

fn main() {
    let mut line = [0u8; 1024];
    let mut len = 0;
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut byte = [0u8; 1];
    while len < 1023 {
        match handle.read(&mut byte) {
            Ok(1) => {
                line[len] = byte[0];
                len += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if len == 0 {
        return;
    }

    let mut end = len;
    for i in 0..len {
        if line[i] == b'\r' || line[i] == b'\n' {
            end = i;
            break;
        }
    }

    let bytes = &line[..end];
    let mut idx = 0;

    while idx < bytes.len() {
        let b = bytes[idx];
        if b == b' ' || b == b'\t' {
            idx += 1;
        } else if b == b'{' || b == b'}' {
            println!("BRACE:{}", b as char);
            idx += 1;
        } else if b == b'[' || b == b']' {
            println!("BRACKET:{}", b as char);
            idx += 1;
        } else if b == b':' {
            println!("COLON::");
            idx += 1;
        } else if b == b',' {
            println!("COMMA:,");
            idx += 1;
        } else if b == b'"' {
            idx += 1;
            let mut buf = Vec::new();
            while idx < bytes.len() && bytes[idx] != b'"' && buf.len() < 255 {
                buf.push(bytes[idx]);
                idx += 1;
            }
            if idx < bytes.len() && bytes[idx] == b'"' {
                idx += 1;
            }
            let s = String::from_utf8_lossy(&buf);
            println!("STRING:{}", s);
        } else if b.is_ascii_digit() || b == b'-' {
            let mut buf = Vec::new();
            while idx < bytes.len()
                && (bytes[idx].is_ascii_digit() || bytes[idx] == b'-' || bytes[idx] == b'.')
                && buf.len() < 63
            {
                buf.push(bytes[idx]);
                idx += 1;
            }
            let s = String::from_utf8_lossy(&buf);
            println!("NUMBER:{}", s);
        } else if bytes[idx..].starts_with(b"true") {
            println!("BOOL:true");
            idx += 4;
        } else if bytes[idx..].starts_with(b"false") {
            println!("BOOL:false");
            idx += 5;
        } else if bytes[idx..].starts_with(b"null") {
            println!("NULL:null");
            idx += 4;
        } else {
            idx += 1;
        }
    }
}
