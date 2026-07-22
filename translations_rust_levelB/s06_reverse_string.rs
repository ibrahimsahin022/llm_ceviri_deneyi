use std::io::{self, Read, Write};

// Seviye B duzeltme: basarisiz test girdisinin cok baytli (UTF-8, Turkce)
// karakterler icerdigi gorulebiliyor (diff/beklenen-cikti gorulmedi). Bu sinyal,
// .chars().rev() yerine bayt bazinda ters cevirmeyi dusundurdu.
fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let line_end = input.iter().position(|&b| b == b'\n').unwrap_or(input.len());
    let mut line = input[..line_end].to_vec();
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    line.reverse();
    let stdout = io::stdout();
    let mut h = stdout.lock();
    h.write_all(&line).unwrap();
    h.write_all(b"\n").unwrap();
}
