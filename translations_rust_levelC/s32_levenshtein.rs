use std::io::{self, Read};

fn levenshtein(s: &[u8], t: &[u8]) -> i32 {
    let ls = s.len();
    let lt = t.len();

    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    if s[ls - 1] == t[lt - 1] {
        return levenshtein(&s[..ls - 1], &t[..lt - 1]);
    }

    let a = levenshtein(&s[..ls - 1], &t[..lt - 1]);
    let b = levenshtein(s, &t[..lt - 1]);
    let c = levenshtein(&s[..ls - 1], t);

    let mut min = a;
    if min > b {
        min = b;
    }
    if min > c {
        min = c;
    }
    min + 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let s1 = lines.next().unwrap_or("").trim();
    let s2 = lines.next().unwrap_or("").trim();

    println!("{}", levenshtein(s1.as_bytes(), s2.as_bytes()));
}
