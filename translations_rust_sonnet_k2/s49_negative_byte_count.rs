use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }

    let total = input.len() as i32;
    let mut negative_count: i32 = 0;
    for &b in &input {
        if (b as i8) < 0 {
            negative_count += 1;
        }
    }

    println!("{} {}", total, negative_count);
}
