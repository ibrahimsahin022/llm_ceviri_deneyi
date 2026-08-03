use std::io;

#[derive(Clone)]
struct Record {
    name: String,
    score: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let n: usize = parts[0].parse().unwrap_or(0);
    let mode = parts[1];

    if n == 0 || n > 1000 {
        return;
    }

    let mut records = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let name = parts[0].to_string();
        let score: i32 = parts[1].parse().unwrap_or(0);
        records.push(Record { name, score });
    }

    if mode == "NAME" {
        records.sort_by(|a, b| a.name.cmp(&b.name));
    } else {
        records.sort_by(|a, b| {
            match a.score.cmp(&b.score) {
                std::cmp::Ordering::Equal => a.name.cmp(&b.name),
                other => other,
            }
        });
    }

    for record in &records {
        println!("{} {}", record.name, record.score);
    }
}
