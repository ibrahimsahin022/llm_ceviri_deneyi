use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq)]
enum UrlReject {
    RejectNada,
    RejectCtrl,
    RejectZero,
}

fn is_xdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hexval(c: char) -> u8 {
    if c >= '0' && c <= '9' {
        (c as u8 - b'0') as u8
    } else if c >= 'a' && c <= 'f' {
        (c as u8 - b'a' + 10) as u8
    } else if c >= 'A' && c <= 'F' {
        (c as u8 - b'A' + 10) as u8
    } else {
        0
    }
}

fn curl_urldecode(string: &str, ctrl: UrlReject) -> Option<String> {
    let mut result = String::new();
    let mut chars = string.chars().peekable();

    while let Some(c) = chars.next() {
        let byte = if c == '%' {
            // Peek ahead to see if we have 2 more hex digits
            let mut lookahead = chars.clone();
            if let (Some(c1), Some(c2)) = (lookahead.next(), lookahead.next()) {
                if is_xdigit(c1) && is_xdigit(c2) {
                    let hex_byte = (hexval(c1) << 4) | hexval(c2);
                    chars.next(); // consume c1
                    chars.next(); // consume c2
                    hex_byte
                } else {
                    c as u8
                }
            } else {
                c as u8
            }
        } else {
            c as u8
        };

        if (ctrl == UrlReject::RejectCtrl && byte < 0x20)
            || (ctrl == UrlReject::RejectZero && byte == 0)
        {
            return None;
        }

        result.push(byte as char);
    }

    Some(result)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Some(decoded) = curl_urldecode(&line, UrlReject::RejectNada) {
            println!("{}", decoded);
        }
    }
}
