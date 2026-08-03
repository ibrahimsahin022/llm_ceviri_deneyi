use std::io::{self, Read};

static SQLITE3_UTF8_TRANS1: [u8; 64] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x00, 0x01, 0x02, 0x03, 0x00, 0x01, 0x00, 0x00,
];

fn sqlite3_utf8_read(pz: &mut &[u8]) -> u32 {
    let mut c = pz[0] as u32;
    *pz = &pz[1..];
    if c >= 0xc0 {
        c = SQLITE3_UTF8_TRANS1[(c - 0xc0) as usize] as u32;
        while !pz.is_empty() && (pz[0] & 0xc0) == 0x80 {
            c = c.wrapping_shl(6).wrapping_add((pz[0] & 0x3f) as u32);
            *pz = &pz[1..];
        }
        if c < 0x80 || (c & 0xffff_f800) == 0xd800 || (c & 0xffff_fffe) == 0xfffe {
            c = 0xfffd;
        }
    }
    c
}

fn main() {
    let mut line = [0u8; 512];
    let mut stdin = io::stdin().lock();
    let mut count = 0;

    while count < 511 {
        let mut buf = [0u8; 1];
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                line[count] = buf[0];
                count += 1;
                if buf[0] == b'\n' {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    if count == 0 {
        return;
    }

    let mut len = count;
    for i in 0..count {
        if line[i] == b'\r' || line[i] == b'\n' || line[i] == 0 {
            len = i;
            break;
        }
    }

    let mut z = &line[..len];
    let mut first = true;
    while !z.is_empty() {
        let c = sqlite3_utf8_read(&mut z);
        if !first {
            print!(" ");
        }
        print!("{}", c);
        first = false;
    }
    println!();
}
