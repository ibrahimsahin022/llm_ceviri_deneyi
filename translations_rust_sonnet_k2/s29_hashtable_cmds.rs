use std::collections::HashMap;
use std::io::{self, Read};

fn is_space(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn atoi_like(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let n = bytes.len();
    while i < n && is_space(bytes[i]) {
        i += 1;
    }
    let start = i;
    if i < n && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < n && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return 0;
    }
    s[start..i].parse::<i64>().unwrap_or(0)
}

fn atol_like(s: &str) -> i64 {
    atoi_like(s)
}

fn read_line(input: &[u8], pos: &mut usize) -> Option<Vec<u8>> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let mut end = *pos;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    *pos = end;
    Some(input[start..end].to_vec())
}

fn trim_newline(line: &mut Vec<u8>) {
    while let Some(&last) = line.last() {
        if last == b'\n' || last == b'\r' {
            line.pop();
        } else {
            break;
        }
    }
}

fn scan_word(bytes: &[u8], pos: &mut usize, maxlen: usize) -> Option<String> {
    let n = bytes.len();
    while *pos < n && is_space(bytes[*pos]) {
        *pos += 1;
    }
    if *pos >= n {
        return None;
    }
    let start = *pos;
    let mut count = 0usize;
    while *pos < n && !is_space(bytes[*pos]) && count < maxlen {
        *pos += 1;
        count += 1;
    }
    Some(String::from_utf8_lossy(&bytes[start..*pos]).into_owned())
}

fn scan_rest(bytes: &[u8], pos: &mut usize) -> Option<String> {
    let n = bytes.len();
    if *pos >= n {
        return None;
    }
    if bytes[*pos] == b'\n' {
        return None;
    }
    let start = *pos;
    while *pos < n && bytes[*pos] != b'\n' {
        *pos += 1;
    }
    if *pos == start {
        None
    } else {
        Some(String::from_utf8_lossy(&bytes[start..*pos]).into_owned())
    }
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;

    let first_line = match read_line(&input, &mut pos) {
        Some(l) => l,
        None => return,
    };
    let k = atoi_like(&String::from_utf8_lossy(&first_line));

    let mut table: HashMap<String, i64> = HashMap::new();

    let mut i: i64 = 0;
    while i < k {
        let mut line = match read_line(&input, &mut pos) {
            Some(l) => l,
            None => break,
        };
        trim_newline(&mut line);

        let mut scan_pos = 0usize;
        let cmd = scan_word(&line, &mut scan_pos, 15);
        let key = if cmd.is_some() {
            scan_word(&line, &mut scan_pos, 63)
        } else {
            None
        };
        let rest = if key.is_some() {
            scan_rest(&line, &mut scan_pos)
        } else {
            None
        };

        let matched = if cmd.is_none() {
            0
        } else if key.is_none() {
            1
        } else if rest.is_none() {
            2
        } else {
            3
        };

        let cmd_s = cmd.unwrap_or_default();
        let key_s = key.unwrap_or_default();
        let rest_s = rest.unwrap_or_default();

        if cmd_s == "INSERT" && matched >= 2 {
            let value = if matched >= 3 { atol_like(&rest_s) } else { 0 };
            table.insert(key_s, value);
            println!("OK");
        } else if cmd_s == "GET" && matched >= 2 {
            match table.get(&key_s) {
                Some(v) => println!("{}", v),
                None => println!("NOT_FOUND"),
            }
        } else if cmd_s == "DEL" && matched >= 2 {
            if table.remove(&key_s).is_some() {
                println!("OK");
            } else {
                println!("NOT_FOUND");
            }
        }

        i += 1;
    }
}
