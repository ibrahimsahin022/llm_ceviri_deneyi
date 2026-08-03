use std::io::{self, Read};

fn stringmatchlen_impl(
    mut pattern: &[u8],
    mut string: &[u8],
    nocase: bool,
    skip_longer_matches: &mut bool,
    nesting: i32,
) -> bool {
    if nesting > 1000 {
        return false;
    }

    while !pattern.is_empty() && !string.is_empty() {
        match pattern[0] {
            b'*' => {
                while pattern.len() >= 2 && pattern[1] == b'*' {
                    pattern = &pattern[1..];
                }
                if pattern.len() == 1 {
                    return true;
                }
                while !string.is_empty() {
                    if stringmatchlen_impl(
                        &pattern[1..],
                        string,
                        nocase,
                        skip_longer_matches,
                        nesting + 1,
                    ) {
                        return true;
                    }
                    if *skip_longer_matches {
                        return false;
                    }
                    string = &string[1..];
                }
                *skip_longer_matches = true;
                return false;
            }
            b'?' => {
                string = &string[1..];
            }
            b'[' => {
                pattern = &pattern[1..];
                let not_flag = !pattern.is_empty() && pattern[0] == b'^';
                if not_flag {
                    pattern = &pattern[1..];
                }
                let mut match_found = false;
                loop {
                    if pattern.len() >= 2 && pattern[0] == b'\\' {
                        pattern = &pattern[1..];
                        if pattern[0] == string[0] {
                            match_found = true;
                        }
                    } else if pattern.is_empty() {
                        break;
                    } else if pattern[0] == b']' {
                        break;
                    } else if pattern.len() >= 3 && pattern[1] == b'-' {
                        let mut start = pattern[0];
                        let mut end = pattern[2];
                        let mut c = string[0];
                        if start > end {
                            std::mem::swap(&mut start, &mut end);
                        }
                        if nocase {
                            start = start.to_ascii_lowercase();
                            end = end.to_ascii_lowercase();
                            c = c.to_ascii_lowercase();
                        }
                        pattern = &pattern[2..];
                        if c >= start && c <= end {
                            match_found = true;
                        }
                    } else {
                        if !nocase {
                            if pattern[0] == string[0] {
                                match_found = true;
                            }
                        } else {
                            if pattern[0].to_ascii_lowercase() == string[0].to_ascii_lowercase() {
                                match_found = true;
                            }
                        }
                    }
                    pattern = &pattern[1..];
                }
                if not_flag {
                    match_found = !match_found;
                }
                if !match_found {
                    return false;
                }
                string = &string[1..];
            }
            b'\\' => {
                if pattern.len() >= 2 {
                    pattern = &pattern[1..];
                }
                if !nocase {
                    if pattern[0] != string[0] {
                        return false;
                    }
                } else {
                    if pattern[0].to_ascii_lowercase() != string[0].to_ascii_lowercase() {
                        return false;
                    }
                }
                string = &string[1..];
            }
            _ => {
                if !nocase {
                    if pattern[0] != string[0] {
                        return false;
                    }
                } else {
                    if pattern[0].to_ascii_lowercase() != string[0].to_ascii_lowercase() {
                        return false;
                    }
                }
                string = &string[1..];
            }
        }

        if !pattern.is_empty() {
            pattern = &pattern[1..];
        }

        if string.is_empty() {
            while !pattern.is_empty() && pattern[0] == b'*' {
                pattern = &pattern[1..];
            }
            break;
        }
    }

    pattern.is_empty() && string.is_empty()
}

fn stringmatchlen(pattern: &[u8], string: &[u8], nocase: bool) -> bool {
    let mut skip_longer_matches = false;
    stringmatchlen_impl(pattern, string, nocase, &mut skip_longer_matches, 0)
}

fn read_line() -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut handle = io::stdin().lock();
    let mut b = [0u8; 1];

    while buf.len() < 255 {
        match handle.read(&mut b) {
            Ok(1) => {
                buf.push(b[0]);
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }

    if buf.is_empty() {
        return None;
    }

    if let Some(pos) = buf.iter().position(|&c| c == b'\r' || c == b'\n') {
        buf.truncate(pos);
    }

    Some(buf)
}

fn main() {
    let pattern = match read_line() {
        Some(p) => p,
        None => return,
    };
    let text = match read_line() {
        Some(t) => t,
        None => return,
    };

    let r = stringmatchlen(&pattern, &text, false);
    if r {
        println!("MATCH");
    } else {
        println!("NOMATCH");
    }
}
