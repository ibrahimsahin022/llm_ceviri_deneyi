use std::io::{self, BufRead};

fn strcasestr(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let needle_lower = needle.to_ascii_lowercase();
    let haystack_lower = haystack.to_ascii_lowercase();

    haystack_lower.find(&needle_lower)
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut hay = String::new();
    let mut needle = String::new();

    if handle.read_line(&mut hay).is_ok() && handle.read_line(&mut needle).is_ok() {
        hay = hay.trim_end_matches(&['\r', '\n'][..]).to_string();
        needle = needle.trim_end_matches(&['\r', '\n'][..]).to_string();

        match strcasestr(&hay, &needle) {
            Some(idx) => println!("{}", idx),
            None => println!("-1"),
        }
    }
}
