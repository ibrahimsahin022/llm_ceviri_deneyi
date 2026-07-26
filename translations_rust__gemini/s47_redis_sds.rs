use std::io::{self, BufRead, Read};

fn sdstrim(s: &mut Vec<u8>, cset: &[u8]) {
    if s.is_empty() {
        return;
    }
    let mut start = 0;
    let mut end = s.len();
    while start < s.len() && cset.contains(&s[start]) {
        start += 1;
    }
    while end > start && cset.contains(&s[end - 1]) {
        end -= 1;
    }
    if start >= end {
        s.clear();
    } else {
        if start > 0 {
            s.drain(..start);
        }
        s.truncate(end - start);
    }
}

fn sdssubstr(s: &mut Vec<u8>, mut start: usize, mut len: usize) {
    let oldlen = s.len();
    if start >= oldlen {
        start = 0;
        len = 0;
    }
    if len > oldlen - start {
        len = oldlen - start;
    }
    if len > 0 {
        s.copy_within(start..start + len, 0);
    }
    s.truncate(len);
}

fn sdsrange(s: &mut Vec<u8>, mut start: isize, mut end: isize) {
    let len = s.len() as isize;
    if len == 0 {
        return;
    }
    if start < 0 {
        start = len + start;
    }
    if end < 0 {
        end = len + end;
    }
    let newlen = if start > end { 0 } else { (end - start) + 1 };
    sdssubstr(s, start as usize, newlen as usize);
}

fn sdstolower(s: &mut [u8]) {
    for b in s.iter_mut() {
        *b = b.to_ascii_lowercase();
    }
}

fn sdstoupper(s: &mut [u8]) {
    for b in s.iter_mut() {
        *b = b.to_ascii_uppercase();
    }
}

fn sdscmp(s1: &[u8], s2: &[u8]) -> i32 {
    let l1 = s1.len();
    let l2 = s2.len();
    let minlen = l1.min(l2);
    for i in 0..minlen {
        if s1[i] != s2[i] {
            return (s1[i] as i32) - (s2[i] as i32);
        }
    }
    if l1 > l2 {
        1
    } else if l1 < l2 {
        -1
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buf = Vec::new();
    let mut b = [0u8; 1];
    loop {
        if handle.read_exact(&mut b).is_err() {
            return;
        }
        if !b[0].is_ascii_whitespace() {
            buf.push(b[0]);
            break;
        }
    }
    loop {
        let available = match handle.fill_buf() {
            Ok(n) => n,
            Err(_) => return,
        };
        if available.is_empty() {
            break;
        }
        let byte = available[0];
        if byte.is_ascii_digit() {
            buf.push(byte);
            handle.consume(1);
        } else {
            handle.consume(1);
            break;
        }
    }
    let ncmd: i32 = match std::str::from_utf8(&buf).ok().and_then(|s| s.parse().ok()) {
        Some(n) => n,
        None => return,
    };

    let mut cur = Vec::<u8>::new();

    for _ in 0..ncmd {
        let mut line_bytes = Vec::new();
        let bytes_read = match handle.read_until(b'\n', &mut line_bytes) {
            Ok(n) => n,
            Err(_) => break,
        };
        if bytes_read == 0 {
            break;
        }

        if line_bytes.len() > 4095 {
            line_bytes.truncate(4095);
        }

        while let Some(&last) = line_bytes.last() {
            if last == b'\n' || last == b'\r' {
                line_bytes.pop();
            } else {
                break;
            }
        }

        let (cmd_bytes, arg_bytes) = if let Some(pos) = line_bytes.iter().position(|&b| b == b' ') {
            let clen = pos.min(15);
            (&line_bytes[..clen], &line_bytes[pos + 1..])
        } else {
            let clen = line_bytes.len().min(15);
            (&line_bytes[..clen], &line_bytes[line_bytes.len()..])
        };

        let cmd = std::str::from_utf8(cmd_bytes).unwrap_or("");

        if cmd == "NEW" {
            cur = arg_bytes.to_vec();
        } else if cmd == "CAT" {
            cur.extend_from_slice(arg_bytes);
        } else if cmd == "TRIM" {
            sdstrim(&mut cur, arg_bytes);
        } else if cmd == "RANGE" {
            let arg_str = std::str::from_utf8(arg_bytes).unwrap_or("");
            let mut parts = arg_str.split_whitespace();
            let a: isize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let b: isize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            sdsrange(&mut cur, a, b);
        } else if cmd == "LOWER" {
            sdstolower(&mut cur);
        } else if cmd == "UPPER" {
            sdstoupper(&mut cur);
        } else if cmd == "CMP" {
            let tmp = arg_bytes.to_vec();
            println!("CMP={}", sdscmp(&cur, &tmp));
            continue;
        }

        let cur_str = std::str::from_utf8(&cur).unwrap_or("");
        println!("LEN={} STR={}", cur.len(), cur_str);
    }
}
