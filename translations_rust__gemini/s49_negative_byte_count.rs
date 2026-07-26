use std::io::{self, Read};

fn main() {
    let mut total: i32 = 0;
    let mut negative_count: i32 = 0;

    for b in io::stdin().lock().bytes() {
        if let Ok(byte) = b {
            let ch = byte as i8;
            if ch < 0 {
                negative_count = negative_count.wrapping_add(1);
            }
            total = total.wrapping_add(1);
        }
    }

    println!("{} {}", total, negative_count);
}
