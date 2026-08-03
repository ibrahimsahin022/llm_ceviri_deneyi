use std::io::{self, BufRead};

fn my_strnstr(s: &[u8], find: &[u8], slen: usize) -> Option<usize> {
    if find.is_empty() {
        return Some(0);
    }

    let c = find[0];
    let search_str = if find.len() > 1 { &find[1..] } else { &[] };
    let len = search_str.len();

    let mut s_idx = 0;
    let mut remaining = slen;

    loop {
        // Search for the first character
        loop {
            if remaining < 1 || s_idx >= s.len() {
                return None;
            }
            let sc = s[s_idx];
            s_idx += 1;
            remaining -= 1;
            if sc == 0 {
                return None;
            }
            if sc == c {
                break;
            }
        }

        // Check if there are enough characters left for the rest
        if len > remaining {
            return None;
        }

        // Compare the rest of the pattern
        let mut match_found = true;
        for i in 0..len {
            if s_idx + i >= s.len() || s[s_idx + i] != search_str[i] {
                match_found = false;
                break;
            }
            if s[s_idx + i] == 0 {
                match_found = false;
                break;
            }
        }

        if match_found {
            return Some(s_idx - 1);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    let mut find = String::new();
    let mut slen_str = String::new();

    if stdin.read_line(&mut s).is_err() {
        return;
    }
    if stdin.read_line(&mut find).is_err() {
        return;
    }
    if stdin.read_line(&mut slen_str).is_err() {
        return;
    }

    s = s.trim_end().to_string();
    find = find.trim_end().to_string();
    let mut slen: i32 = slen_str.trim().parse().unwrap_or(0);

    if slen < 0 {
        slen = 0;
    }

    match my_strnstr(s.as_bytes(), find.as_bytes(), slen as usize) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}
