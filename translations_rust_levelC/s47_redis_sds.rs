use std::io::{self, BufRead, Write};

/// LLM cevirisi notu: C orijinali, degisken genislikte bir baslik
/// (len/alloc/flags) pointer'in HEMEN ONCESINE gizlenmis sekilde saklayan
/// "sds" pointer temsilini kullaniyordu (bkz. samples_c/s47_redis_sds.c
/// basindaki aciklama). Bu, Rust'ta pointer-oncesi-baslik duzenine dogal
/// bir karsilik olmadigi icin, burada dogrudan buyume/kucculme mantigini
/// (trim/range/case donusumu, MakeRoomFor tarzi otomatik buyume) koruyan
/// ama ic temsili String olan guvenli bir yeniden yapilandirma tercih
/// edildi. Bu ayni zamanda calismanin ana sorularindan birini test eder:
/// LLM, C'nin bellek duzeni detaylarini (sdshdr8/16/32/64 tur secimi gibi)
/// yeniden uretmeye mi calisir, yoksa gozlemlenebilir davranisi (API
/// sozlesmesini) koruyup ic temsili tamamen mi degistirir?
struct Sds {
    buf: String,
}

impl Sds {
    fn empty() -> Self {
        Sds { buf: String::new() }
    }
    fn new_from(s: &str) -> Self {
        Sds { buf: s.to_string() }
    }
    fn cat(&mut self, t: &str) {
        self.buf.push_str(t);
    }
    fn trim(&mut self, cset: &str) {
        let trimmed = self.buf.trim_matches(|c| cset.contains(c));
        self.buf = trimmed.to_string();
    }
    fn range(&mut self, start: i64, end: i64) {
        let len = self.buf.len() as i64;
        if len == 0 {
            return;
        }
        let mut start = start;
        let mut end = end;
        if start < 0 {
            start = len + start;
        }
        if end < 0 {
            end = len + end;
        }
        let newlen: i64 = if start > end { 0 } else { end - start + 1 };
        self.substr(start, newlen);
    }
    fn substr(&mut self, start: i64, len: i64) {
        let oldlen = self.buf.len() as i64;
        let mut start = start;
        let mut len = len;
        if start >= oldlen || start < 0 {
            start = 0;
            len = 0;
        }
        if len > oldlen - start {
            len = oldlen - start;
        }
        if len < 0 {
            len = 0;
        }
        let s = start as usize;
        let l = len as usize;
        self.buf = self.buf[s..s + l].to_string();
    }
    fn to_lower(&mut self) {
        self.buf = self.buf.to_lowercase();
    }
    fn to_upper(&mut self) {
        self.buf = self.buf.to_uppercase();
    }
    fn cmp_to(&self, other: &Sds) -> i32 {
        match self.buf.as_bytes().cmp(other.buf.as_bytes()) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let ncmd: usize = lines.next().unwrap().trim().parse().unwrap();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut cur = Sds::empty();
    for _ in 0..ncmd {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };
        let (cmd, arg) = match line.find(' ') {
            Some(idx) => (&line[..idx], &line[idx + 1..]),
            None => (line.as_str(), ""),
        };

        match cmd {
            "NEW" => {
                cur = Sds::new_from(arg);
            }
            "CAT" => {
                cur.cat(arg);
            }
            "TRIM" => {
                cur.trim(arg);
            }
            "RANGE" => {
                let mut parts = arg.split_whitespace();
                let a: i64 = parts.next().unwrap().parse().unwrap();
                let b: i64 = parts.next().unwrap().parse().unwrap();
                cur.range(a, b);
            }
            "LOWER" => {
                cur.to_lower();
            }
            "UPPER" => {
                cur.to_upper();
            }
            "CMP" => {
                let tmp = Sds::new_from(arg);
                writeln!(out, "CMP={}", cur.cmp_to(&tmp)).unwrap();
                continue;
            }
            _ => {}
        }
        writeln!(out, "LEN={} STR={}", cur.buf.len(), cur.buf).unwrap();
    }
}
