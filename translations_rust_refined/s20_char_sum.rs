use std::io::{self, Read};

// IYILESTIRME (Round 2): C'de 'char' isaretlidir; 127'den buyuk baytlar negatiftir.
// Ilk ceviri baytlari u8 (0..255, hepsi pozitif) olarak topladigi icin cok baytli
// girdide C'den farkli sonuc veriyordu (FE). C ile ayni davranis icin her bayt
// once i8'e (isaretli) donusturulur.
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
