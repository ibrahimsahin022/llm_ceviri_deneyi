use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = Vec::new();

    match handle.read_until(b'\n', &mut buf) {
        Ok(0) => return,
        Ok(_) => {}
        Err(_) => return,
    }

    if buf.len() > 4095 {
        buf.truncate(4095);
    }

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    let len = buf.len();
    let mut i = 0;
    while i < len {
        let mut j = i;
        while j < len && buf[j] == buf[i] {
            j += 1;
        }
        print!("{}{}", buf[i] as char, j - i);
        i = j;
    }
    println!();
}
