use std::io::{self, Read};

const SQLITE_MATCH: i32 = 0;
const SQLITE_NOMATCH: i32 = 1;
const SQLITE_NOWILDCARDMATCH: i32 = 2;

struct CompareInfo {
    match_all: u8,
    match_one: u8,
    match_set: u8,
    no_case: u8,
}

static GLOB_INFO: CompareInfo = CompareInfo {
    match_all: b'*',
    match_one: b'?',
    match_set: b'[',
    no_case: 0,
};

static SQLITE3_UTF8_TRANS1: [u8; 64] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x00, 0x01, 0x02, 0x03, 0x00, 0x01, 0x00, 0x00,
];

unsafe fn sqlite3_utf8_read(pz: &mut *const u8) -> u32 {
    let mut c = **pz as u32;
    *pz = pz.add(1);
    if c >= 0xc0 {
        c = SQLITE3_UTF8_TRANS1[(c - 0xc0) as usize] as u32;
        while (**pz & 0xc0) == 0x80 {
            c = (c << 6) + (0x3f & (**pz as u32));
            *pz = pz.add(1);
        }
        if c < 0x80 || (c & 0xFFFF_F800) == 0xD800 || (c & 0xFFFF_FFFE) == 0xFFFE {
            c = 0xFFFD;
        }
    }
    c
}

fn sqlite3_toupper(x: u32) -> u32 {
    if x >= 'a' as u32 && x <= 'z' as u32 {
        x - 32
    } else {
        x
    }
}

fn sqlite3_tolower(x: u32) -> u32 {
    if x >= 'A' as u32 && x <= 'Z' as u32 {
        x + 32
    } else {
        x
    }
}

unsafe fn sqlite_skip_utf8(z: &mut *const u8) {
    let old = **z;
    *z = z.add(1);
    if old >= 0xc0 {
        while (**z & 0xc0) == 0x80 {
            *z = z.add(1);
        }
    }
}

unsafe fn strcspn_zstop(mut s: *const u8, z_stop: &[u8]) -> usize {
    let mut count = 0;
    while *s != 0 {
        if z_stop.contains(&*s) {
            break;
        }
        count += 1;
        s = s.add(1);
    }
    count
}

unsafe fn pattern_compare(
    mut z_pattern: *const u8,
    mut z_string: *const u8,
    p_info: &CompareInfo,
    match_other: u32,
) -> i32 {
    let mut c: u32;
    let mut c2: u32;
    let match_one = p_info.match_one as u32;
    let match_all = p_info.match_all as u32;
    let no_case = p_info.no_case != 0;
    let mut z_escaped: *const u8 = std::ptr::null();

    while {
        c = sqlite3_utf8_read(&mut z_pattern);
        c != 0
    } {
        if c == match_all {
            loop {
                c = sqlite3_utf8_read(&mut z_pattern);
                if !(c == match_all || (c == match_one && match_one != 0)) {
                    break;
                }
                if c == match_one && sqlite3_utf8_read(&mut z_string) == 0 {
                    return SQLITE_NOWILDCARDMATCH;
                }
            }
            if c == 0 {
                return SQLITE_MATCH;
            } else if c == match_other {
                if p_info.match_set == 0 {
                    c = sqlite3_utf8_read(&mut z_pattern);
                    if c == 0 {
                        return SQLITE_NOWILDCARDMATCH;
                    }
                } else {
                    while *z_string != 0 {
                        let b_match = pattern_compare(
                            z_pattern.offset(-1),
                            z_string,
                            p_info,
                            match_other,
                        );
                        if b_match != SQLITE_NOMATCH {
                            return b_match;
                        }
                        sqlite_skip_utf8(&mut z_string);
                    }
                    return SQLITE_NOWILDCARDMATCH;
                }
            }

            if c < 0x80 {
                let z_stop: &[u8] = if no_case {
                    &[
                        sqlite3_toupper(c) as u8,
                        sqlite3_tolower(c) as u8,
                    ]
                } else {
                    &[c as u8]
                };
                loop {
                    let skip = strcspn_zstop(z_string, z_stop);
                    z_string = z_string.add(skip);
                    if *z_string == 0 {
                        break;
                    }
                    z_string = z_string.add(1);
                    let b_match = pattern_compare(z_pattern, z_string, p_info, match_other);
                    if b_match != SQLITE_NOMATCH {
                        return b_match;
                    }
                }
            } else {
                while {
                    c2 = sqlite3_utf8_read(&mut z_string);
                    c2 != 0
                } {
                    if c2 != c {
                        continue;
                    }
                    let b_match = pattern_compare(z_pattern, z_string, p_info, match_other);
                    if b_match != SQLITE_NOMATCH {
                        return b_match;
                    }
                }
            }
            return SQLITE_NOWILDCARDMATCH;
        }
        if c == match_other {
            if p_info.match_set == 0 {
                c = sqlite3_utf8_read(&mut z_pattern);
                if c == 0 {
                    return SQLITE_NOMATCH;
                }
                z_escaped = z_pattern;
            } else {
                let mut prior_c: u32 = 0;
                let mut seen = 0;
                let mut invert = 0;
                c = sqlite3_utf8_read(&mut z_string);
                if c == 0 {
                    return SQLITE_NOMATCH;
                }
                c2 = sqlite3_utf8_read(&mut z_pattern);
                if c2 == '^' as u32 {
                    invert = 1;
                    c2 = sqlite3_utf8_read(&mut z_pattern);
                }
                if c2 == ']' as u32 {
                    if c == ']' as u32 {
                        seen = 1;
                    }
                    c2 = sqlite3_utf8_read(&mut z_pattern);
                }
                while c2 != 0 && c2 != ']' as u32 {
                    if c2 == '-' as u32 && *z_pattern != ']' as u8 && *z_pattern != 0 && prior_c > 0 {
                        c2 = sqlite3_utf8_read(&mut z_pattern);
                        if c >= prior_c && c <= c2 {
                            seen = 1;
                        }
                        prior_c = 0;
                    } else {
                        if c == c2 {
                            seen = 1;
                        }
                        prior_c = c2;
                    }
                    c2 = sqlite3_utf8_read(&mut z_pattern);
                }
                if c2 == 0 || (seen ^ invert) == 0 {
                    return SQLITE_NOMATCH;
                }
                continue;
            }
        }
        c2 = sqlite3_utf8_read(&mut z_string);
        if c == c2 {
            continue;
        }
        if no_case && sqlite3_tolower(c) == sqlite3_tolower(c2) && c < 0x80 && c2 < 0x80 {
            continue;
        }
        if c == match_one && z_pattern != z_escaped && c2 != 0 {
            continue;
        }
        return SQLITE_NOMATCH;
    }
    if *z_string == 0 {
        SQLITE_MATCH
    } else {
        SQLITE_NOMATCH
    }
}

fn fgets_256() -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut byte = [0u8; 1];
    while buf.len() < 255 {
        match handle.read(&mut byte) {
            Ok(1) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }
    if buf.is_empty() {
        return None;
    }
    if let Some(pos) = buf.iter().position(|&b| b == b'\r' || b == b'\n') {
        buf.truncate(pos);
    }
    buf.push(0);
    Some(buf)
}

fn main() {
    let pattern = match fgets_256() {
        Some(p) => p,
        None => return,
    };
    let text = match fgets_256() {
        Some(t) => t,
        None => return,
    };
    let r = unsafe {
        pattern_compare(
            pattern.as_ptr(),
            text.as_ptr(),
            &GLOB_INFO,
            '[' as u32,
        )
    };
    if r == SQLITE_MATCH {
        println!("MATCH");
    } else {
        println!("NOMATCH");
    }
}
