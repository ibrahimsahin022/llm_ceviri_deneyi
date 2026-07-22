mod config;
mod lookup;
mod parser;

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut config_text = String::new();
    for _ in 0..n {
        if let Some(l) = lines.next() {
            config_text.push_str(l);
            config_text.push('\n');
        }
    }

    let entries = parser::parse_config_lines(&config_text);

    let m: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..m {
        let key = match lines.next() {
            Some(l) => l.trim(),
            None => break,
        };
        match lookup::config_lookup(&entries, key) {
            Some(v) => println!("{}", v),
            None => println!("NOT_FOUND"),
        }
    }
}
