// SEVIYE B duzeltme denemesi: modele yalnizca basarisiz test girdisi
// verildi ("cgiosu", "Merhaba dunya, Istanbul cok guzel!" - cok baytli
// Turkce karakterler icerir), beklenen/alinan cikti farki GOSTERILMEDI.
// Girdideki cok baytli (UTF-8) karakterler, C'nin `char` isaretliligiyle
// ilgili bir sorunu akla getirdigi icin, baytin once i8'e (isaretli)
// donusturulmesi deneniyor.
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut negative_count = 0;
    let mut total = 0;
    for &b in &buf {
        let ch: i32 = (b as i8) as i32;
        if ch < 0 {
            negative_count += 1;
        }
        total += 1;
    }
    println!("{} {}", total, negative_count);
}
