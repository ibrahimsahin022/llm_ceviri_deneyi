use std::io::{self, BufRead};

const UTF8_TRANS1: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x00, 0x01, 0x02, 0x03, 0x00, 0x01, 0x00, 0x00,
];

fn sqlite3_utf8_read(data: &[u8], pos: &mut usize) -> u32 {
    if *pos >= data.len() {
        return 0;
    }

    let c = data[*pos] as u32;
    *pos += 1;

    if c >= 0xc0 {
        let idx = (c - 0xc0) as usize;
        let mut c = if idx < UTF8_TRANS1.len() {
            UTF8_TRANS1[idx] as u32
        } else {
            0
        };

        while *pos < data.len() && (data[*pos] & 0xc0) == 0x80 {
            c = (c << 6) + (0x3f & data[*pos] as u32);
            *pos += 1;
        }

        if c < 0x80 || (c & 0xFFFFF800) == 0xD800 || (c & 0xFFFFFFFE) == 0xFFFE {
            c = 0xFFFD;
        }

        c
    } else {
        c
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let data = line.as_bytes();
        let mut pos = 0;
        let mut first = true;

        while pos < data.len() {
            let c = sqlite3_utf8_read(data, &mut pos);
            if c == 0 {
                break;
            }
            if !first {
                print!(" ");
            }
            print!("{}", c);
            first = false;
        }
        println!();
    }
}
