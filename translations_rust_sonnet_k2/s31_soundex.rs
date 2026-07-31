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

fn build_code_table() -> [i32; 128] {
    let mut code = [0i32; 128];
    let classes: [(&str, i32); 8] = [
        ("AEIOU", -1),
        ("", 0),
        ("BFPV", 1),
        ("CGJKQSXZ", 2),
        ("DT", 3),
        ("L", 4),
        ("MN", 5),
        ("R", 6),
    ];
    for (cls, val) in classes.iter() {
        for &b in cls.as_bytes() {
            code[b as usize] = *val;
            code[(b ^ 0x20) as usize] = *val;
        }
    }
    code
}

fn soundex(s: &[u8], code: &[i32; 128]) -> Vec<u8> {
    let mut out = vec![0u8; 5];
    if s.is_empty() {
        return out;
    }
    out[0] = s[0];
    let mut idx = 1usize;
    let mut prev = code[s[0] as usize];
    let mut i = 1usize;
    while idx < s.len() && i < 4 {
        let c = code[s[idx] as usize];
        if c == prev {
            idx += 1;
            continue;
        }
        if c == -1 {
            prev = 0;
        } else if c > 0 {
            out[i] = (c as u8) + b'0';
            i += 1;
            prev = c;
        }
        idx += 1;
    }
    while i < 4 {
        out[i] = b'0';
        i += 1;
    }
    out
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;

    let mut buf = match fgets_sim(&input, &mut pos, 128) {
        Some(l) => l,
        None => return,
    };

    while let Some(&last) = buf.last() {
        if last == b'\n' || last == b'\r' {
            buf.pop();
        } else {
            break;
        }
    }

    let code = build_code_table();
    let out = soundex(&buf, &code);

    // print as null-terminated string (stop at first 0 byte, max 4 chars)
    let mut result = String::new();
    for &b in &out[..4] {
        if b == 0 {
            break;
        }
        result.push(b as char);
    }
    println!("{}", result);
}
