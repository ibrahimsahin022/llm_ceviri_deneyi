use std::io::{self, BufRead};

fn stringmatchlen_impl(
    pattern: &[u8],
    string: &[u8],
    nocase: bool,
    skip_longer_matches: &mut bool,
    nesting: i32,
) -> bool {
    if nesting > 1000 {
        return false;
    }

    let mut pi = 0usize;
    let mut si = 0usize;
    let plen0 = pattern.len();
    let slen0 = string.len();

    while pi < plen0 && si < slen0 {
        match pattern[pi] {
            b'*' => {
                while pi + 1 < plen0 && pattern[pi + 1] == b'*' {
                    pi += 1;
                }
                if pi == plen0 - 1 {
                    return true;
                }
                while si < slen0 {
                    if stringmatchlen_impl(&pattern[pi + 1..], &string[si..], nocase, skip_longer_matches, nesting + 1) {
                        return true;
                    }
                    if *skip_longer_matches {
                        return false;
                    }
                    si += 1;
                }
                *skip_longer_matches = true;
                return false;
            }
            b'?' => {
                si += 1;
            }
            b'[' => {
                pi += 1;
                let mut not = false;
                if pi < plen0 && pattern[pi] == b'^' {
                    not = true;
                    pi += 1;
                }
                let mut is_match = false;
                loop {
                    if pi + 1 < plen0 && pattern[pi] == b'\\' {
                        pi += 1;
                        if pattern[pi] == string[si] {
                            is_match = true;
                        }
                    } else if pi >= plen0 {
                        pi -= 1;
                        break;
                    } else if pattern[pi] == b']' {
                        break;
                    } else if pi + 2 < plen0 && pattern[pi + 1] == b'-' {
                        let mut start = pattern[pi];
                        let mut end = pattern[pi + 2];
                        let mut c = string[si];
                        if start > end {
                            std::mem::swap(&mut start, &mut end);
                        }
                        if nocase {
                            start = start.to_ascii_lowercase();
                            end = end.to_ascii_lowercase();
                            c = c.to_ascii_lowercase();
                        }
                        pi += 2;
                        if c >= start && c <= end {
                            is_match = true;
                        }
                    } else {
                        if !nocase {
                            if pattern[pi] == string[si] {
                                is_match = true;
                            }
                        } else if pattern[pi].to_ascii_lowercase() == string[si].to_ascii_lowercase() {
                            is_match = true;
                        }
                    }
                    pi += 1;
                }
                if not {
                    is_match = !is_match;
                }
                if !is_match {
                    return false;
                }
                si += 1;
            }
            b'\\' => {
                if pi + 1 < plen0 {
                    pi += 1;
                }
                if !nocase {
                    if pattern[pi] != string[si] {
                        return false;
                    }
                } else if pattern[pi].to_ascii_lowercase() != string[si].to_ascii_lowercase() {
                    return false;
                }
                si += 1;
            }
            _ => {
                if !nocase {
                    if pattern[pi] != string[si] {
                        return false;
                    }
                } else if pattern[pi].to_ascii_lowercase() != string[si].to_ascii_lowercase() {
                    return false;
                }
                si += 1;
            }
        }
        pi += 1;
        if si == slen0 {
            while pi < plen0 && pattern[pi] == b'*' {
                pi += 1;
            }
            break;
        }
    }

    pi == plen0 && si == slen0
}

fn stringmatchlen(pattern: &[u8], string: &[u8], nocase: bool) -> bool {
    let mut skip_longer_matches = false;
    stringmatchlen_impl(pattern, string, nocase, &mut skip_longer_matches, 0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let pattern = lines.next().unwrap().unwrap();
    let text = lines.next().unwrap().unwrap();
    let r = stringmatchlen(pattern.as_bytes(), text.as_bytes(), false);
    println!("{}", if r { "MATCH" } else { "NOMATCH" });
}
