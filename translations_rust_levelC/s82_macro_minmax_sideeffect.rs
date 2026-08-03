use std::io::{self, Read};

// Seviye C: "4/4 test basarisiz" - her girdide yanlis, yani x'in son
// degeri her zaman yanlis olmali.
// Makrolari metinsel olarak actim:
//   m = MIN(x++, y)  ->  ((x++) < (y) ? (x++) : (y))
//   M = MAX(x++, y)  ->  ((x++) > (y) ? (x++) : (y))
// Ucluk operatorunun kosulundan sonra bir siralama noktasi vardir, yani
// davranis tanimlidir: kosulda x++ BIR KEZ calisir; kosul DOGRU ise
// secilen dalda x++ IKINCI KEZ calisir ve degeri o anki (bir artmis) x
// olur; kosul YANLIS ise sonuc y'dir ve x yalnizca bir kez artar.
// Round 1 cevirisi ise argumani her zaman TAM BIR KEZ degerlendirmis
// (x += 1 sabit olarak iki kez), bu yuzden hem m/M hem de son x yanlis.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();
    let mut x: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let y: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // int m = MIN(x++, y);  ->  ((x++) < y ? (x++) : y)
    let m: i32;
    let cond_a = x; // x++ kosulda: eski deger kullanilir
    x += 1;
    if cond_a < y {
        m = x; // ikinci x++: o anki (artmis) deger sonuctur
        x += 1;
    } else {
        m = y;
    }

    // int M = MAX(x++, y);  ->  ((x++) > y ? (x++) : y)
    let mm: i32;
    let cond_b = x;
    x += 1;
    if cond_b > y {
        mm = x;
        x += 1;
    } else {
        mm = y;
    }

    println!("{} {} {}", m, mm, x);
}
