use std::io::{self, Read};

// Seviye B duzeltme: basarisiz test girdisinin cok baytli (UTF-8, Turkce)
// karakterler icerdigi gorulebiliyor (diff/beklenen-cikti gorulmedi). Bu sinyal,
// karakter sayimi yerine bayt sayimina gecmeyi dusundurdu.
fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut chars = 0;
    let mut words = 0;
    let mut inword = false;
    for &b in &input {
        chars += 1;
        if b == b' ' || b == b'\n' || b == b'\t' || b == b'\r' {
            inword = false;
        } else if !inword {
            words += 1;
            inword = true;
        }
    }
    println!("{} {}", words, chars);
}
