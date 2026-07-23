use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut negative_count = 0;
    let mut total = 0;
    for &b in &buf {
        let ch: i32 = b as i32;
        if ch < 0 {
            negative_count += 1;
        }
        total += 1;
    }
    println!("{} {}", total, negative_count);
}
