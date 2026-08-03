use std::io::{self, Read};

struct Record {
    name: String,
    score: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mode = it.next().unwrap().to_string();

    let mut records = Vec::with_capacity(n);
    for _ in 0..n {
        let name = it.next().unwrap().to_string();
        let score: i32 = it.next().unwrap().parse().unwrap();
        records.push(Record { name, score });
    }

    if mode == "NAME" {
        records.sort_by(|a, b| a.name.cmp(&b.name));
    } else {
        records.sort_by(|a, b| {
            if a.score != b.score {
                a.score.cmp(&b.score)
            } else {
                a.name.cmp(&b.name)
            }
        });
    }

    for r in &records {
        println!("{} {}", r.name, r.score);
    }
}
