use std::io::{self, Read};

// Seviye B duzeltme: basarisiz test girdisinin 127'den buyuk bayt degerleri
// icerdigi gorulebiliyor (diff/beklenen-cikti gorulmedi). C'de 'char' cogu
// platformda isaretlidir; bu sinyal isaretli (i8) donusume gecmeyi dusundurdu.
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut sum: i64 = 0;
    for b in line.bytes() {
        sum += (b as i8) as i64;
    }
    println!("{}", sum);
}
