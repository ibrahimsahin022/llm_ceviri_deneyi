use std::io::{self, Read};

fn levenshtein(s: &[u8], ls: i64, t: &[u8], lt: i64) -> i64 {
    if ls == 0 {
        return lt;
    }
    if lt == 0 {
        return ls;
    }

    if s[(ls - 1) as usize] == t[(lt - 1) as usize] {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    let a = levenshtein(s, ls - 1, t, lt - 1);
    let b = levenshtein(s, ls, t, lt - 1);
    let c = levenshtein(s, ls - 1, t, lt);

    let mut a = a;
    if a > b {
        a = b;
    }
    if a > c {
        a = c;
    }

    a + 1
}

fn fgets_sim(input: &[u8], pos: &mut usize, cap: usize) -> Option<Vec<u8>> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let maxdata = cap - 1;
    let mut count = 0usize;
    let mut end = *pos;
    while end < input.len() && count < maxdata {
        if input[end] == b'\n' {
            end += 1;
            break;
        }
        end += 1;
        count += 1;
    }
    *pos = end;
    Some(input[start..end].to_vec())
}

fn trim_newline(s: &mut Vec<u8>) {
    while let Some(&last) = s.last() {
        if last == b'\n' || last == b'\r' {
            s.pop();
        } else {
            break;
        }
    }
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;

    let mut s1 = match fgets_sim(&input, &mut pos, 64) {
        Some(l) => l,
        None => return,
    };
    let mut s2 = match fgets_sim(&input, &mut pos, 64) {
        Some(l) => l,
        None => return,
    };
    trim_newline(&mut s1);
    trim_newline(&mut s2);

    let l1 = s1.len() as i64;
    let l2 = s2.len() as i64;

    println!("{}", levenshtein(&s1, l1, &s2, l2));
}
