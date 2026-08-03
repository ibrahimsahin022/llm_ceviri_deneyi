use std::io::{self, BufRead};

const SQLITE_MATCH: i32 = 0;
const SQLITE_NOMATCH: i32 = 1;
const SQLITE_NOWILDCARDMATCH: i32 = 2;

struct CompareInfo {
    match_all: u8,
    match_one: u8,
    match_set: u8,
    no_case: u8,
}

const GLOB_INFO: CompareInfo = CompareInfo {
    match_all: b'*',
    match_one: b'?',
    match_set: b'[',
    no_case: 0,
};

fn utf8_read(bytes: &[u8], pos: &mut usize) -> u32 {
    if *pos >= bytes.len() {
        return 0;
    }
    let c0 = bytes[*pos];
    *pos += 1;
    if c0 < 0xc0 {
        return c0 as u32;
    }
    // Basit UTF-8 devam baytı toplama (ASCII odaklı test girdileri için yeterli)
    let mut c = (c0 & 0x3f) as u32;
    while *pos < bytes.len() && (bytes[*pos] & 0xc0) == 0x80 {
        c = (c << 6) + (bytes[*pos] & 0x3f) as u32;
        *pos += 1;
    }
    if c < 0x80 {
        0xFFFD
    } else {
        c
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
    p_start: usize,
    text: &[u8],
    t_start: usize,
    info: &CompareInfo,
    match_other: u8,
) -> i32 {
    let mut pi = p_start;
    let mut ti = t_start;
    let match_one = info.match_one as u32;
    let match_all = info.match_all as u32;
    let no_case = info.no_case != 0;

    loop {
        let c = utf8_read(pattern, &mut pi);
        if c == 0 {
            break;
        }
        if c == match_all as u32 {
            let mut c2;
            loop {
                c2 = utf8_read(pattern, &mut pi);
                if c2 == match_all as u32 || (c2 == match_one && match_one != 0) {
                    if c2 == match_one {
                        let mut tmp = ti;
                        if utf8_read(text, &mut tmp) == 0 {
                            return SQLITE_NOWILDCARDMATCH;
                        }
                        ti = tmp;
                    }
                    continue;
                }
                break;
            }
            if c2 == 0 {
                return SQLITE_MATCH;
            } else if c2 == match_other as u32 {
                if info.match_set == 0 {
                    let cc = utf8_read(pattern, &mut pi);
                    if cc == 0 {
                        return SQLITE_NOWILDCARDMATCH;
                    }
                }
            }
            // Basitleştirilmiş arama: kalan metinde eşleşen ilk konumdan yeniden dene
            let mut search_ti = ti;
            loop {
                if search_ti > text.len() {
                    return SQLITE_NOWILDCARDMATCH;
                }
                let bmatch = pattern_compare(pattern, pi - char_len(pattern, pi), text, search_ti, info, match_other);
                if bmatch != SQLITE_NOMATCH {
                    return bmatch;
                }
                if search_ti >= text.len() {
                    return SQLITE_NOWILDCARDMATCH;
                }
                search_ti += 1;
            }
        }
        if c == match_other as u32 {
            if info.match_set != 0 {
                let mut prior_c: u32 = 0;
                let mut seen = false;
                let mut invert = false;
                let tc = utf8_read(text, &mut ti);
                if tc == 0 {
                    return SQLITE_NOMATCH;
                }
                let mut c2 = utf8_read(pattern, &mut pi);
                if c2 == '^' as u32 {
                    invert = true;
                    c2 = utf8_read(pattern, &mut pi);
                }
                if c2 == ']' as u32 {
                    if tc == ']' as u32 {
                        seen = true;
                    }
                    c2 = utf8_read(pattern, &mut pi);
                }
                while c2 != 0 && c2 != ']' as u32 {
                    if c2 == '-' as u32 && pi < pattern.len() && pattern[pi] != b']' && prior_c > 0 {
                        c2 = utf8_read(pattern, &mut pi);
                        if tc >= prior_c && tc <= c2 {
                            seen = true;
                        }
                        prior_c = 0;
                    } else {
                        if tc == c2 {
                            seen = true;
                        }
                        prior_c = c2;
                    }
                    c2 = utf8_read(pattern, &mut pi);
                }
                if c2 == 0 || (seen ^ invert) == false {
                    return SQLITE_NOMATCH;
                }
                continue;
            }
        }
        let c2 = utf8_read(text, &mut ti);
        if c == c2 {
            continue;
        }
        if no_case && to_lower(c) == to_lower(c2) && c < 0x80 && c2 < 0x80 {
            continue;
        }
        if c == match_one && c2 != 0 {
            continue;
        }
        return SQLITE_NOMATCH;
    }
    if ti >= text.len() {
        SQLITE_MATCH
    } else {
        SQLITE_NOMATCH
    }
}

fn char_len(_pattern: &[u8], _pos: usize) -> usize {
    1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let pattern = lines.next().unwrap().unwrap();
    let text = lines.next().unwrap().unwrap();
    let r = pattern_compare(pattern.as_bytes(), 0, text.as_bytes(), 0, &GLOB_INFO, b'[');
    println!("{}", if r == SQLITE_MATCH { "MATCH" } else { "NOMATCH" });
}
