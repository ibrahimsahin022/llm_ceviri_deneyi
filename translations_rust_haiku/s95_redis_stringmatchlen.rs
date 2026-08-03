use std::io::{self, BufRead};

fn stringmatchlen_impl(
    pattern: &[u8],
    string: &[u8],
    nocase: bool,
    skip_longer_matches: &mut bool,
    nesting: usize,
) -> bool {
    if nesting > 1000 {
        return false;
    }

    let mut p_idx = 0;
    let mut s_idx = 0;

    while p_idx < pattern.len() && s_idx < string.len() {
        match pattern[p_idx] {
            b'*' => {
                p_idx += 1;
                while p_idx < pattern.len() && pattern[p_idx] == b'*' {
                    p_idx += 1;
                }
                if p_idx == pattern.len() {
                    return true;
                }
                loop {
                    if stringmatchlen_impl(
                        &pattern[p_idx..],
                        &string[s_idx..],
                        nocase,
                        skip_longer_matches,
                        nesting + 1,
                    ) {
                        return true;
                    }
                    if *skip_longer_matches {
                        return false;
                    }
                    s_idx += 1;
                    if s_idx >= string.len() {
                        break;
                    }
                }
                *skip_longer_matches = true;
                return false;
            }
            b'?' => {
                p_idx += 1;
                s_idx += 1;
            }
            b'[' => {
                p_idx += 1;
                let not = p_idx < pattern.len() && pattern[p_idx] == b'^';
                if not {
                    p_idx += 1;
                }
                let mut match_found = false;
                loop {
                    if p_idx + 1 < pattern.len() && pattern[p_idx] == b'\\' {
                        p_idx += 1;
                        if pattern[p_idx] == string[s_idx] {
                            match_found = true;
                        }
                    } else if p_idx >= pattern.len() {
                        p_idx -= 1;
                        break;
                    } else if pattern[p_idx] == b']' {
                        break;
                    } else if p_idx + 2 < pattern.len() && pattern[p_idx + 1] == b'-' {
                        let start_c = pattern[p_idx];
                        let end_c = pattern[p_idx + 2];
                        let mut start = if nocase {
                            start_c.to_ascii_lowercase()
                        } else {
                            start_c
                        };
                        let mut end = if nocase {
                            end_c.to_ascii_lowercase()
                        } else {
                            end_c
                        };
                        let mut c = if nocase {
                            string[s_idx].to_ascii_lowercase()
                        } else {
                            string[s_idx]
                        };

                        if start > end {
                            std::mem::swap(&mut start, &mut end);
                        }
                        p_idx += 2;
                        if c >= start && c <= end {
                            match_found = true;
                        }
                    } else {
                        let pat_char = pattern[p_idx];
                        let str_char = string[s_idx];
                        if nocase {
                            if pat_char.to_ascii_lowercase() == str_char.to_ascii_lowercase() {
                                match_found = true;
                            }
                        } else {
                            if pat_char == str_char {
                                match_found = true;
                            }
                        }
                    }
                    p_idx += 1;
                }
                if not {
                    match_found = !match_found;
                }
                if !match_found {
                    return false;
                }
                s_idx += 1;
            }
            b'\\' => {
                if p_idx + 1 < pattern.len() {
                    p_idx += 1;
                }
                let pat_char = pattern[p_idx];
                let str_char = string[s_idx];
                if nocase {
                    if pat_char.to_ascii_lowercase() != str_char.to_ascii_lowercase() {
                        return false;
                    }
                } else {
                    if pat_char != str_char {
                        return false;
                    }
                }
                s_idx += 1;
                p_idx += 1;
            }
            _ => {
                let pat_char = pattern[p_idx];
                let str_char = string[s_idx];
                if nocase {
                    if pat_char.to_ascii_lowercase() != str_char.to_ascii_lowercase() {
                        return false;
                    }
                } else {
                    if pat_char != str_char {
                        return false;
                    }
                }
                s_idx += 1;
                p_idx += 1;
            }
        }

        if s_idx >= string.len() {
            while p_idx < pattern.len() && pattern[p_idx] == b'*' {
                p_idx += 1;
            }
            break;
        }
    }

    p_idx == pattern.len() && s_idx == string.len()
}

fn stringmatchlen(pattern: &[u8], string: &[u8]) -> bool {
    let mut skip_longer_matches = false;
    stringmatchlen_impl(pattern, string, false, &mut skip_longer_matches, 0)
}

fn main() {
    let stdin = io::stdin();
    let mut pattern = String::new();
    let mut text = String::new();

    if stdin.read_line(&mut pattern).is_err() {
        return;
    }
    if stdin.read_line(&mut text).is_err() {
        return;
    }

    pattern = pattern.trim_end().to_string();
    text = text.trim_end().to_string();

    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    let result = stringmatchlen(pattern_bytes, text_bytes);
    println!("{}", if result { "MATCH" } else { "NOMATCH" });
}
