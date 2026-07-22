use std::io::{self, BufRead, Write};

fn peek_byte<R: BufRead>(reader: &mut R) -> Option<u8> {
    let buf = reader.fill_buf().ok()?;
    buf.first().copied()
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut byte = match peek_byte(&mut reader) {
        Some(b) => b,
        None => return,
    };

    while is_space(byte) {
        reader.consume(1);
        match peek_byte(&mut reader) {
            Some(b) => byte = b,
            None => return,
        }
    }

    let mut negative = false;
    if byte == b'-' {
        negative = true;
        reader.consume(1);
        byte = match peek_byte(&mut reader) {
            Some(b) => b,
            None => return,
        };
    } else if byte == b'+' {
        reader.consume(1);
        byte = match peek_byte(&mut reader) {
            Some(b) => b,
            None => return,
        };
    }

    if !byte.is_ascii_digit() {
        return;
    }

    let mut k: i32 = 0;
    while byte.is_ascii_digit() {
        k = k.wrapping_mul(10).wrapping_add((byte - b'0') as i32);
        reader.consume(1);
        match peek_byte(&mut reader) {
            Some(b) => byte = b,
            None => break,
        }
    }

    if negative {
        k = k.wrapping_neg();
    }

    while let Some(b) = peek_byte(&mut reader) {
        if is_space(b) {
            reader.consume(1);
        } else {
            break;
        }
    }

    let k = ((k % 26) + 26) % 26;

    let mut buf = Vec::new();
    while buf.len() < 4095 {
        match peek_byte(&mut reader) {
            Some(b) => {
                reader.consume(1);
                buf.push(b);
                if b == b'\n' {
                    break;
                }
            }
            None => break,
        }
    }

    if buf.is_empty() {
        return;
    }

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());

    for &c in &buf {
        if c >= b'a' && c <= b'z' {
            let shifted = b'a' + (((c - b'a') as i32 + k) % 26) as u8;
            let _ = writer.write_all(&[shifted]);
        } else if c >= b'A' && c <= b'Z' {
            let shifted = b'A' + (((c - b'A') as i32 + k) % 26) as u8;
            let _ = writer.write_all(&[shifted]);
        } else if c != b'\n' && c != b'\r' {
            let _ = writer.write_all(&[c]);
        }
    }
    let _ = writer.write_all(b"\n");
    let _ = writer.flush();
}
