use std::io::{self, BufRead};

fn sodium_bin2hex(bin: &[u8]) -> Option<String> {
    let hex_maxlen = bin.len() * 2 + 1;
    let mut hex = String::with_capacity(hex_maxlen);

    if hex_maxlen <= bin.len() * 2 {
        return None;
    }

    for byte in bin {
        let c = (byte & 0xf) as u32;
        let b = (byte >> 4) as u32;

        // Constant-time hex encoding using wrapping arithmetic
        // For each nibble: if < 10, output '0'-'9' (48-57), else 'a'-'f' (97-102)
        let c_digit = {
            let diff = c.wrapping_sub(10);
            let selector = (diff >> 8) & !38;
            (87u32 + c + selector) as u8 as char
        };

        let b_digit = {
            let diff = b.wrapping_sub(10);
            let selector = (diff >> 8) & !38;
            (87u32 + b + selector) as u8 as char
        };

        hex.push(b_digit);
        hex.push(c_digit);
    }

    Some(hex)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let bin = line.as_bytes();
        if let Some(result) = sodium_bin2hex(bin) {
            println!("{}", result);
        }
    }
}
