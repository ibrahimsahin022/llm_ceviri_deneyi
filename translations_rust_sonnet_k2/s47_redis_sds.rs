use std::io::{self, Read};

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

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn parse_cmd_arg(line: &[u8]) -> (String, Vec<u8>) {
    if let Some(pos) = line.iter().position(|&b| b == b' ') {
        let mut clen = pos;
        if clen > 15 {
            clen = 15;
        }
        let cmd = String::from_utf8_lossy(&line[..clen]).into_owned();
        let arg = line[pos + 1..].to_vec();
        (cmd, arg)
    } else {
        let clen = line.len().min(15);
        let cmd = String::from_utf8_lossy(&line[..clen]).into_owned();
        (cmd, Vec::new())
    }
}

fn memcmp_bytes(a: &[u8], b: &[u8]) -> i32 {
    let minlen = a.len().min(b.len());
    for i in 0..minlen {
        if a[i] != b[i] {
            return a[i] as i32 - b[i] as i32;
        }
    }
    0
}

fn sdscmp(a: &[u8], b: &[u8]) -> i32 {
    let l1 = a.len();
    let l2 = b.len();
    let minlen = l1.min(l2);
    let cmp = memcmp_bytes(&a[..minlen], &b[..minlen]);
    if cmp == 0 {
        if l1 > l2 {
            1
        } else if l1 < l2 {
            -1
        } else {
            0
        }
    } else {
        cmp
    }
}

fn sdstrim(s: &mut Vec<u8>, cset: &[u8]) {
    if s.is_empty() {
        return;
    }
    let mut sp = 0usize;
    let mut ep = s.len() - 1;
    while sp < s.len() && sp <= ep && cset.contains(&s[sp]) {
        sp += 1;
    }
    if sp > ep {
        // entire string trimmed away
        s.clear();
        return;
    }
    while ep > sp && cset.contains(&s[ep]) {
        ep -= 1;
    }
    let new_content = s[sp..=ep].to_vec();
    *s = new_content;
}

fn sdssubstr(s: &mut Vec<u8>, start: i64, len: i64) {
    let oldlen = s.len() as i64;
    let mut start = start;
    let mut len = len;
    if start < 0 || start >= oldlen {
        start = 0;
        len = 0;
    }
    if len > oldlen - start {
        len = oldlen - start;
    }
    if len < 0 {
        len = 0;
    }
    let start_u = start as usize;
    let len_u = len as usize;
    let new_content = s[start_u..start_u + len_u].to_vec();
    *s = new_content;
}

fn sdsrange(s: &mut Vec<u8>, start: i64, end: i64) {
    let len = s.len() as i64;
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
    let newlen = if start > end { 0 } else { (end - start) + 1 };
    sdssubstr(s, start, newlen);
}

fn parse_two_longs(arg: &[u8]) -> (i64, i64) {
    // mimic sscanf(arg, "%ld %ld", &a, &b) — missing values default to 0
    let mut idx = 0usize;
    let n = arg.len();

    let mut parse_one = |idx: &mut usize| -> Option<i64> {
        while *idx < n && is_space_ws(arg[*idx]) {
            *idx += 1;
        }
        let start = *idx;
        if *idx < n && (arg[*idx] == b'+' || arg[*idx] == b'-') {
            *idx += 1;
        }
        let digits_start = *idx;
        while *idx < n && arg[*idx].is_ascii_digit() {
            *idx += 1;
        }
        if *idx == digits_start {
            return None;
        }
        std::str::from_utf8(&arg[start..*idx])
            .ok()
            .and_then(|s| s.parse::<i64>().ok())
    };

    let a = parse_one(&mut idx).unwrap_or(0);
    let b = parse_one(&mut idx).unwrap_or(0);
    (a, b)
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;
    let total_len = input.len();

    while pos < total_len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let sign_start = pos;
    if pos < total_len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let digits_start = pos;
    while pos < total_len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if digits_start == pos {
        return;
    }
    let ncmd: i32 = match std::str::from_utf8(&input[sign_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    if pos < total_len {
        pos += 1; // getchar()
    }

    let mut cur: Vec<u8> = Vec::new();

    for _ in 0..ncmd {
        let mut line = match fgets_sim(&input, &mut pos, 4096) {
            Some(l) => l,
            None => break,
        };
        trim_newline(&mut line);

        let (cmd, arg) = parse_cmd_arg(&line);

        if cmd == "NEW" {
            cur = arg.clone();
        } else if cmd == "CAT" {
            cur.extend_from_slice(&arg);
        } else if cmd == "TRIM" {
            sdstrim(&mut cur, &arg);
        } else if cmd == "RANGE" {
            let (a, b) = parse_two_longs(&arg);
            sdsrange(&mut cur, a, b);
        } else if cmd == "LOWER" {
            cur.make_ascii_lowercase();
        } else if cmd == "UPPER" {
            cur.make_ascii_uppercase();
        } else if cmd == "CMP" {
            let tmp = arg.clone();
            println!("CMP={}", sdscmp(&cur, &tmp));
            continue;
        }

        println!("LEN={} STR={}", cur.len(), String::from_utf8_lossy(&cur));
    }
}
