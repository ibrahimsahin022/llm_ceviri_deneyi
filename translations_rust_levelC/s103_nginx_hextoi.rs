use std::io::{self, Read};

// Seviye C: yalnizca "1/4 test basarisiz" bilgisi var.
// ngx_hextoi'nin tasma korumasi NGX_MAX_INT_T_VALUE = LONG_MAX uzerinden
// calisiyor. Kaynak yorumu tiplerin `long`a esitlendigini soyluyor ve bu
// ortam (Windows/LLP64, bkz. diger orneklerdeki platform notlari) `long`u
// 32 BIT yapar -> LONG_MAX = 2147483647, cutoff = 134217727.
// Round 1 cevirisi i64 (cutoff = 576460752303423487) secmis; bu yuzden
// 2^31'i asan buyuk hex girdilerde C -1 dondururken Rust degeri basiyor.
// Tek bir testin basarisiz olmasi tam da bu "buyuk deger" senaryosuna
// isaret ediyor.
// Ek olarak: bos girdide C hicbir sey basmadan cikar (fgets basarisiz);
// Round 1 cevirisi orada unwrap ile panige gider.

fn ngx_hextoi(line: &[u8]) -> i32 {
    let n = line.len();
    if n == 0 {
        return -1; // NGX_ERROR
    }
    let cutoff: i32 = i32::MAX / 16;
    let mut value: i32 = 0;
    for &ch in line {
        if value > cutoff {
            return -1;
        }
        if ch >= b'0' && ch <= b'9' {
            value = value * 16 + (ch - b'0') as i32;
            continue;
        }
        let c = ch | 0x20;
        if c >= b'a' && c <= b'f' {
            value = value * 16 + (c - b'a' + 10) as i32;
            continue;
        }
        return -1;
    }
    value
}

fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    if buf.is_empty() {
        return; // fgets NULL dondurur -> cikti yok
    }
    // fgets ile tek satir + strcspn(line, "\r\n") ile kirpma
    let mut end = match buf.iter().position(|&b| b == b'\n') {
        Some(p) => p + 1,
        None => buf.len(),
    };
    // C'de satir buf[128] ile sinirli
    if end > 127 {
        end = 127;
    }
    let mut line = &buf[..end];
    while !line.is_empty() {
        let last = line[line.len() - 1];
        if last == b'\n' || last == b'\r' {
            line = &line[..line.len() - 1];
        } else {
            break;
        }
    }
    println!("{}", ngx_hextoi(line));
}
