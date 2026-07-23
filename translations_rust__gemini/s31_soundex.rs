use std::io::{self, Read};

fn init() -> [i8; 128] {
    let mut code = [0i8; 128];
    let cls = ["AEIOU", "", "BFPV", "CGJKQSXZ", "DT", "L", "MN", "R"];
    for (i, &s) in cls.iter().enumerate() {
        let c = (i as i8) - 1;
        for &b in s.as_bytes() {
            let idx1 = b as usize;
            let idx2 = (0x20 ^ b) as usize;
            if idx1 < 128 {
                code[idx1] = c;
            }
            if idx2 < 128 {
                code[idx2] = c;
            }
        }
    }
    code
}

fn soundex(s: &[u8], code: &[i8; 128]) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut out = [0u8; 4];
    out[0] = s[0];

    let get_code = |b: u8| -> i8 {
        if (b as usize) < 128 {
            code[b as usize]
        } else {
            0
        }
    };

    let mut prev = get_code(out[0]);
    let mut i = 1;

    for &b in &s[1..] {
        if i >= 4 {
            break;
        }
        let c = get_code(b);
        if c == prev {
            continue;
        }

        if c == -1 {
            prev = 0;
        } else if c > 0 {
            out[i] = (c as u8) + b'0';
            i += 1;
            prev = c;
        }
    }

    while i < 4 {
        out[i] = b'0';
        i += 1;
    }

    String::from_utf8_lossy(&out).into_owned()
}

fn main() {
    let mut buf = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut byte = [0u8; 1];

    while buf.len() < 127 {
        match handle.read(&mut byte) {
            Ok(1) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if buf.is_empty() {
        return;
    }

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    let code = init();
    println!("{}", soundex(&buf, &code));
}
