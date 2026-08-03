use std::io::{self, BufRead};

fn hexval(x: u8) -> u8 {
    match x {
        b'0'..=b'9' => x - b'0',
        b'a'..=b'f' => x - b'a' + 10,
        b'A'..=b'F' => x - b'A' + 10,
        _ => 0,
    }
}

fn is_xdigit(x: u8) -> bool {
    x.is_ascii_hexdigit()
}

fn curl_urldecode(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0usize;
    while i < input.len() {
        let in_byte = input[i];
        if in_byte == b'%' && i + 2 < input.len() && is_xdigit(input[i + 1]) && is_xdigit(input[i + 2]) {
            let decoded = (hexval(input[i + 1]) << 4) | hexval(input[i + 2]);
            out.push(decoded);
            i += 3;
        } else {
            out.push(in_byte);
            i += 1;
        }
    }
    out
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let decoded = curl_urldecode(line.as_bytes());
    println!("{}", String::from_utf8_lossy(&decoded));
}
