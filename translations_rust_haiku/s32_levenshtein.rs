use std::io::{self, BufRead};

fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> i32 {
    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    if s_bytes[ls - 1] == t_bytes[lt - 1] {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    let a = levenshtein(s, ls - 1, t, lt - 1);
    let b = levenshtein(s, ls, t, lt - 1);
    let c = levenshtein(s, ls - 1, t, lt);

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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut s1)) = lines.next() {
        while s1.ends_with('\n') || s1.ends_with('\r') {
            s1.pop();
        }

        if let Some(Ok(mut s2)) = lines.next() {
            while s2.ends_with('\n') || s2.ends_with('\r') {
                s2.pop();
            }

            println!("{}", levenshtein(&s1, s1.len(), &s2, s2.len()));
        }
    }
}
