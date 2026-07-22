use std::io::{self, Read};

// IYILESTIRME (Round 2): C'nin 'static mut' global durumu Rust'ta 'unsafe' olmadan
// derlenmez (E0133). Guvenli ve idiyomatik cozum: sayaci global degil, fonksiyona
// DEGISTIRILEBILIR REFERANS (&mut) olarak gecmek.
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
