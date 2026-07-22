use std::io::{self, Read};

// Seviye B duzeltme: derleyici hatasi (E0133, mutable static) tek basina
// yeterli bir sinyaldir - CE her zaman CI loglarinda tam olarak gorunur.
fn next_id(counter: &mut i32) -> i32 {
    *counter += 1;
    *counter
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut call_count = 0;
    for _ in 0..n {
        println!("{}", next_id(&mut call_count));
    }
}
