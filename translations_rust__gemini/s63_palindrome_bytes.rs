use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 4096];
    let mut read_len = 0;
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    while read_len < 4095 {
        let mut byte = [0u8; 1];
        match handle.read(&mut byte) {
            Ok(1) => {
                buf[read_len] = byte[0];
                read_len += 1;
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if read_len == 0 {
        return;
    }

    let mut len = read_len;
    for i in 0..read_len {
        if buf[i] == 0 {
            len = i;
            break;
        }
    }

    while len > 0 && (buf[len - 1] == b'\n' || buf[len - 1] == b'\r') {
        len -= 1;
    }

    let mut ok = true;
    for i in 0..len / 2 {
        if buf[i] != buf[len - 1 - i] {
            ok = false;
            break;
        }
    }

    if ok {
        println!("EVET");
    } else {
        println!("HAYIR");
    }
}
