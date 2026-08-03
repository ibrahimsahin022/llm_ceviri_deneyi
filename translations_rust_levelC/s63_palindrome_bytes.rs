use std::io::{self, Read};

// Seviye C: yalnizca "1/4 test basarisiz" bilgisi var.
// C kaynagi karsilastirmayi BAYT indeksiyle yapiyor: buf[i] vs
// buf[len-1-i]. Round 1 cevirisi .chars().rev() kullanmis; bu KOD NOKTASI
// bazinda calisir ve cok baytli UTF-8 girdide C'den farkli sonuc verir.
// Ayrica String olarak okumak gecersiz UTF-8'de panige yol acar.
// Cozum: ham baytlarla, C ile birebir ayni indeks aritmetigi.
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    // fgets tek satir okur: ilk '\n' dahil. Sonra sondaki '\n'/'\r' kirpilir.
    let mut len = match buf.iter().position(|&b| b == b'\n') {
        Some(p) => p + 1,
        None => buf.len(),
    };
    while len > 0 && (buf[len - 1] == b'\n' || buf[len - 1] == b'\r') {
        len -= 1;
    }
    let mut ok = true;
    for i in 0..(len / 2) {
        if buf[i] != buf[len - 1 - i] {
            ok = false;
            break;
        }
    }
    println!("{}", if ok { "EVET" } else { "HAYIR" });
}
