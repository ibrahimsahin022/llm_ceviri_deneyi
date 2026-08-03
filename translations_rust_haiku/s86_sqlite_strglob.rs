use std::io::{self, BufRead};

const SQLITE_MATCH: i32 = 0;
const SQLITE_NOMATCH: i32 = 1;
const SQLITE_NOWILDCARDMATCH: i32 = 2;

const UTF8_TRANS1: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];

#[derive(Clone)]
struct CompareInfo {
    match_all: u32,
    match_one: u32,
    match_set: u32,
    no_case: bool,
}

fn utf8_read(data: &[u8], pos: &mut usize) -> u32 {
    if *pos >= data.len() {
        return 0;
    }
    let c = data[*pos] as u32;
    *pos += 1;

    if c >= 0xc0 {
        let trans_idx = (c - 0xc0) as usize;
        let trans = if trans_idx < UTF8_TRANS1.len() {
            UTF8_TRANS1[trans_idx] as u32
        } else {
            0
        };
        let mut c = trans;
        while *pos < data.len() && (data[*pos] & 0xc0) == 0x80 {
            c = (c << 6) + (0x3f & data[*pos] as u32);
            *pos += 1;
        }
        if c < 0x80 || (c & 0xFFFFF800) == 0xD800 || (c & 0xFFFFFFFE) == 0xFFFE {
            c = 0xFFFD;
        }
        c
    } else {
        c
    }
}

fn utf8_skip(data: &[u8], pos: &mut usize) {
    if *pos < data.len() {
        if data[*pos] >= 0xc0 {
            *pos += 1;
            while *pos < data.len() && (data[*pos] & 0xc0) == 0x80 {
                *pos += 1;
            }
        } else {
            *pos += 1;
        }
    }
}

fn to_upper(c: u32) -> u32 {
    if c >= 'a' as u32 && c <= 'z' as u32 {
        c - 32
    } else {
        c
    }
}

fn to_lower(c: u32) -> u32 {
    if c >= 'A' as u32 && c <= 'Z' as u32 {
        c + 32
    } else {
        c
    }
}

fn pattern_compare(
    pattern: &[u8],
    string: &[u8],
    p_start: usize,
    s_start: usize,
    info: &CompareInfo,
    match_other: u32,
) -> i32 {
    let mut p_idx = p_start;
    let mut s_idx = s_start;
    let mut z_escaped: Option<usize> = None;

    loop {
        let c = utf8_read(pattern, &mut p_idx);
        if c == 0 {
            break;
        }

        if c == info.match_all {
            loop {
                let c_inner = utf8_read(pattern, &mut p_idx);
                if c_inner == info.match_all || (c_inner == info.match_one && info.match_one != 0) {
                    if c_inner == info.match_one && utf8_read(string, &mut s_idx) == 0 {
                        return SQLITE_NOWILDCARDMATCH;
                    }
                    continue;
                }

                if c_inner == 0 {
                    return SQLITE_MATCH;
                } else if c_inner == match_other {
                    if info.match_set == 0 {
                        let _ = utf8_read(pattern, &mut p_idx);
                        if p_idx > pattern.len() {
                            return SQLITE_NOWILDCARDMATCH;
                        }
                    } else {
                        loop {
                            if s_idx >= string.len() {
                                break;
                            }
                            let saved_p = p_idx;
                            if saved_p > 0 {
                                let bm = pattern_compare(pattern, string, p_idx - 1, s_idx, info, match_other);
                                if bm != SQLITE_NOMATCH {
                                    return bm;
                                }
                            }
                            utf8_skip(string, &mut s_idx);
                        }
                        return SQLITE_NOWILDCARDMATCH;
                    }
                }

                if c_inner < 0x80 {
                    loop {
                        if s_idx >= string.len() {
                            break;
                        }
                        let mut test_s = s_idx;
                        let test_c = utf8_read(string, &mut test_s);
                        if test_c == c_inner
                            || (info.no_case && to_lower(c_inner) == to_lower(test_c) && c_inner < 0x80 && test_c < 0x80)
                        {
                            s_idx = test_s;
                            let bm = pattern_compare(pattern, string, p_idx, s_idx, info, match_other);
                            if bm != SQLITE_NOMATCH {
                                return bm;
                            }
                        } else {
                            utf8_skip(string, &mut s_idx);
                        }
                    }
                } else {
                    loop {
                        let c2 = utf8_read(string, &mut s_idx);
                        if c2 == 0 {
                            break;
                        }
                        if c2 == c_inner {
                            let bm = pattern_compare(pattern, string, p_idx, s_idx, info, match_other);
                            if bm != SQLITE_NOMATCH {
                                return bm;
                            }
                        }
                    }
                }
                return SQLITE_NOWILDCARDMATCH;
            }
        }

        if c == match_other {
            if info.match_set == 0 {
                let _ = utf8_read(pattern, &mut p_idx);
                if p_idx > pattern.len() {
                    return SQLITE_NOMATCH;
                }
                z_escaped = Some(p_idx);
            } else {
                let mut prior_c: u32 = 0;
                let mut seen = false;
                let mut invert = false;

                let c_match = utf8_read(string, &mut s_idx);
                if c_match == 0 {
                    return SQLITE_NOMATCH;
                }

                let mut c2 = utf8_read(pattern, &mut p_idx);
                if c2 == '^' as u32 {
                    invert = true;
                    c2 = utf8_read(pattern, &mut p_idx);
                }
                if c2 == ']' as u32 {
                    if c_match == ']' as u32 {
                        seen = true;
                    }
                    c2 = utf8_read(pattern, &mut p_idx);
                }

                while c2 != 0 && c2 != ']' as u32 {
                    if c2 == '-' as u32
                        && p_idx < pattern.len()
                        && pattern[p_idx] != ']' as u8
                        && pattern[p_idx] != 0
                        && prior_c > 0
                    {
                        c2 = utf8_read(pattern, &mut p_idx);
                        if c_match >= prior_c && c_match <= c2 {
                            seen = true;
                        }
                        prior_c = 0;
                    } else {
                        if c_match == c2 {
                            seen = true;
                        }
                        prior_c = c2;
                    }
                    c2 = utf8_read(pattern, &mut p_idx);
                }

                if c2 == 0 || (seen ^ invert) == false {
                    return SQLITE_NOMATCH;
                }
                continue;
            }
        }

        let c2 = utf8_read(string, &mut s_idx);
        if c == c2 {
            continue;
        }
        if info.no_case && to_lower(c) == to_lower(c2) && c < 0x80 && c2 < 0x80 {
            continue;
        }
        if c == info.match_one && z_escaped.is_none() && c2 != 0 {
            continue;
        }
        return SQLITE_NOMATCH;
    }

    if s_idx >= string.len() {
        SQLITE_MATCH
    } else {
        SQLITE_NOMATCH
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let pattern_line = match lines.next() {
        Some(Ok(l)) => l,
        _ => return,
    };
    let string_line = match lines.next() {
        Some(Ok(l)) => l,
        _ => return,
    };

    let pattern_bytes = pattern_line.as_bytes();
    let string_bytes = string_line.as_bytes();

    let info = CompareInfo {
        match_all: '*' as u32,
        match_one: '?' as u32,
        match_set: '[' as u32,
        no_case: false,
    };

    let result = pattern_compare(pattern_bytes, string_bytes, 0, 0, &info, '[' as u32);
    println!("{}", if result == SQLITE_MATCH { "MATCH" } else { "NOMATCH" });
}
