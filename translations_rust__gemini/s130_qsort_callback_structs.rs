use std::io::{self, Read};

struct Record {
    name: String,
    score: i32,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mode = match tokens.next() {
        Some(s) => s,
        None => return,
    };

    if n < 0 || n > 1000 {
        return;
    }

    let mut records = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let name_str = match tokens.next() {
            Some(s) => s,
            None => return,
        };
        let name = if name_str.len() > 31 {
            &name_str[..31]
        } else {
            name_str
        };

        let score: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };

        records.push(Record {
            name: name.to_string(),
            score,
        });
    }

    let mode_truncated = if mode.len() > 7 { &mode[..7] } else { mode };

    if mode_truncated == "NAME" {
        records.sort_by(|a, b| a.name.cmp(&b.name));
    } else {
        records.sort_by(|a, b| match a.score.cmp(&b.score) {
            std::cmp::Ordering::Equal => a.name.cmp(&b.name),
            ord => ord,
        });
    }

    for record in &records {
        println!("{} {}", record.name, record.score);
    }
}
