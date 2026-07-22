use std::io::{self, Read, Write};

// IYILESTIRME (Round 2): C programi diziyi BAYT bazinda ters cevirir.
// Ilk cevirideki `.chars().rev()` KARAKTER bazinda ters cevirdigi icin
// cok baytli (UTF-8) girdilerde C'den farkli sonuc uretiyordu (fonksiyonel hata).
// C ile ayni davranis icin bayt bazinda ters cevirme yapildi.
fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let line_end = input.iter().position(|&b| b == b'\n').unwrap_or(input.len());
    let mut line = input[..line_end].to_vec();
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    line.reverse(); // bayt bazinda ters cevir (C ile ayni)
    let stdout = io::stdout();
    let mut h = stdout.lock();
    h.write_all(&line).unwrap();
    h.write_all(b"\n").unwrap();
}
