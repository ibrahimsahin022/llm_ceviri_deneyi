use std::io::{self, BufRead};

fn fgets<R: BufRead>(reader: &mut R, buf: &mut [u8]) -> bool {
    if buf.len() <= 1 {
        return false;
    }
    let max_read = buf.len() - 1;
    let mut count = 0;
    while count < max_read {
        let mut b = [0u8; 1];
        match reader.read(&mut b) {
            Ok(1) => {
                buf[count] = b[0];
                count += 1;
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }
    if count == 0 {
        false
    } else {
        buf[count] = 0;
        true
    }
}

fn strcspn_rn(buf: &[u8]) -> usize {
    for (i, &b) in buf.iter().enumerate() {
        if b == 0 || b == b'\r' || b == b'\n' {
            return i;
        }
    }
    buf.len()
}

fn my_memrchr(s: &[u8], c: u8) -> Option<usize> {
    s.iter().rposition(|&x| x == c)
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut text = [0u8; 512];
    let mut ch = [0u8; 8];

    if !fgets(&mut handle, &mut text) {
        return;
    }
    if !fgets(&mut handle, &mut ch) {
        return;
    }

    let text_len = strcspn_rn(&text);
    text[text_len] = 0;

    let ch_len = strcspn_rn(&ch);
    ch[ch_len] = 0;

    if ch[0] == 0 {
        return;
    }

    let text_slice = &text[..text_len];
    if let Some(pos) = my_memrchr(text_slice, ch[0]) {
        println!("{}", pos);
    } else {
        println!("-1");
    }
}
