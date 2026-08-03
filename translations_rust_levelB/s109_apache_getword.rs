use std::io::{self, BufRead};

fn getword<'a>(line: &mut &'a str, stop: char) -> &'a str {
    let pos = line.find(stop).unwrap_or(line.len());
    let word = &line[..pos];
    let mut rest = &line[pos..];
    while rest.starts_with(stop) {
        rest = &rest[stop.len_utf8()..];
    }
    *line = rest;
    word
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let stop_line = lines.next().unwrap().unwrap();
    let text = lines.next().unwrap().unwrap();
    if stop_line.is_empty() {
        return;
    }
    let stop = stop_line.chars().next().unwrap();

    let mut p: &str = &text;
    while !p.is_empty() {
        let w = getword(&mut p, stop);
        println!("{}", w);
    }
}
