use std::io::{self, BufRead};

fn my_getword<'a>(line: &mut &'a [u8], stop: u8) -> &'a [u8] {
    let mut pos = 0;
    while pos < line.len() && line[pos] != stop {
        pos += 1;
    }

    let res = &line[..pos];

    if stop != 0 {
        while pos < line.len() && line[pos] == stop {
            pos += 1;
        }
    }

    *line = &line[pos..];
    res
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut stopc = Vec::new();
    if handle.read_until(b'\n', &mut stopc).unwrap_or(0) == 0 {
        return;
    }

    let mut text = Vec::new();
    if handle.read_until(b'\n', &mut text).unwrap_or(0) == 0 {
        return;
    }

    if let Some(pos) = stopc.iter().position(|&b| b == b'\r' || b == b'\n') {
        stopc.truncate(pos);
    }
    if let Some(pos) = text.iter().position(|&b| b == b'\r' || b == b'\n') {
        text.truncate(pos);
    }

    if stopc.is_empty() {
        return;
    }

    let stop = stopc[0];
    let mut p = &text[..];

    while !p.is_empty() {
        let w = my_getword(&mut p, stop);
        println!("{}", String::from_utf8_lossy(w));
    }
}
