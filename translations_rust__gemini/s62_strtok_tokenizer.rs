use std::io;

fn main() {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).unwrap_or(0) == 0 {
        return;
    }

    let trimmed = buf.trim_end_matches(|c| c == '\r' || c == '\n');

    for (idx, tok) in trimmed.split(' ').filter(|s| !s.is_empty()).enumerate() {
        println!("{}: {}", idx, tok);
    }
}
