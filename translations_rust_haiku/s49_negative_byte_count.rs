use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).ok();

    let mut negative_count = 0;
    let total = input.len();

    for &byte in &input {
        let ch = byte as i8;
        if ch < 0 {
            negative_count += 1;
        }
    }

    println!("{} {}", total, negative_count);
}
