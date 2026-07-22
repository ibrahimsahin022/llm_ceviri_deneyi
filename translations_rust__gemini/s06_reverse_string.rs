use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = Vec::new();

    if handle.read_until(b'\n', &mut buf).unwrap_or(0) == 0 {
        return;
    }

    if buf.len() > 1023 {
        buf.truncate(1023);
    }

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    buf.reverse();
    buf.push(b'\n');

    let _ = io::stdout().write_all(&buf);
}
