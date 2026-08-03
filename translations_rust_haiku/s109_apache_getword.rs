use std::io::{self, BufRead};

fn getword(line: &mut &str, stop: u8) -> String {
    let line_bytes = line.as_bytes();
    let mut pos = 0;

    // Find the position of the stop character
    while pos < line_bytes.len() && line_bytes[pos] != stop {
        pos += 1;
    }

    let word = &line[..pos];
    *line = &line[pos..];

    // Skip consecutive stop characters
    while !line.is_empty() && line.as_bytes()[0] == stop {
        *line = &line[1..];
    }

    word.to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stopc = String::new();
    let mut text = String::new();

    if handle.read_line(&mut stopc).is_ok() && handle.read_line(&mut text).is_ok() {
        stopc = stopc.trim_end_matches(&['\r', '\n'][..]).to_string();
        text = text.trim_end_matches(&['\r', '\n'][..]).to_string();

        if !stopc.is_empty() {
            let stop_byte = stopc.as_bytes()[0];
            let mut p = text.as_str();

            while !p.is_empty() {
                let w = getword(&mut p, stop_byte);
                println!("{}", w);
            }
        }
    }
}
