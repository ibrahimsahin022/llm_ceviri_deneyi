use std::io::{self, Read};

const MAX_KEY_LEN: usize = 32;
const MAX_VAL_LEN: usize = 128;

#[derive(Clone)]
struct ConfigEntry {
    key: String,
    value: String,
}

fn trim(s: &str) -> String {
    // strip trailing '\n', '\r', ' ' then strip leading ' '
    let mut end = s.len();
    let bytes = s.as_bytes();
    while end > 0 {
        let c = bytes[end - 1];
        if c == b'\n' || c == b'\r' || c == b' ' {
            end -= 1;
        } else {
            break;
        }
    }
    let trimmed_end = &s[..end];
    let start = trimmed_end
        .as_bytes()
        .iter()
        .take_while(|&&b| b == b' ')
        .count();
    trimmed_end[start..].to_string()
}

/// "key=value" satirlarindan olusan metni ayristirir; bos satirlari ve
/// '#' ile baslayan yorum satirlarini atlar.
fn parse_config_lines(text: &str) -> Vec<ConfigEntry> {
    let mut entries = Vec::new();
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut pos = 0usize;

    while pos < n {
        let line_start = pos;
        while pos < n && bytes[pos] != b'\n' {
            pos += 1;
        }
        let raw_line = &text[line_start..pos];
        if pos < n {
            pos += 1; // skip '\n'
        }

        let line = trim(raw_line);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let eq_pos = match line.find('=') {
            Some(p) => p,
            None => continue,
        };

        let key = trim(&line[..eq_pos]);
        let value = trim(&line[eq_pos + 1..]);

        let mut key_trunc = key;
        key_trunc.truncate(MAX_KEY_LEN - 1);
        let mut value_trunc = value;
        value_trunc.truncate(MAX_VAL_LEN - 1);

        entries.push(ConfigEntry {
            key: key_trunc,
            value: value_trunc,
        });
    }

    entries
}

/// key'i bulursa value'ya referans dondurur; bulamazsa None.
fn config_lookup<'a>(entries: &'a [ConfigEntry], key: &str) -> Option<&'a str> {
    for e in entries {
        if e.key == key {
            return Some(&e.value);
        }
    }
    None
}

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

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

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let total_len = input.len();
    let mut pos = 0usize;

    // scanf("%d", &n)
    while pos < total_len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let n_start = pos;
    if pos < total_len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let n_digits_start = pos;
    while pos < total_len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if n_digits_start == pos {
        return;
    }
    let n: i32 = match std::str::from_utf8(&input[n_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    if pos < total_len {
        pos += 1; // getchar()
    }

    // build "text" buffer (max 4095 usable bytes, matching char text[4096])
    let mut text = String::new();
    let text_cap = 4095usize;
    for _ in 0..n {
        let line = match fgets_sim(&input, &mut pos, 256) {
            Some(l) => l,
            None => break,
        };
        let line_str = String::from_utf8_lossy(&line).into_owned();
        if text.len() + line_str.len() < text_cap {
            text.push_str(&line_str);
        }
    }

    let entries = parse_config_lines(&text);

    // scanf("%d", &m)
    while pos < total_len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let m_start = pos;
    if pos < total_len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let m_digits_start = pos;
    while pos < total_len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if m_digits_start == pos {
        return;
    }
    let m: i32 = match std::str::from_utf8(&input[m_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    for _ in 0..m {
        // %31s
        while pos < total_len && is_space_ws(input[pos]) {
            pos += 1;
        }
        let word_start = pos;
        let mut wcount = 0usize;
        while pos < total_len && !is_space_ws(input[pos]) && wcount < 31 {
            pos += 1;
            wcount += 1;
        }
        if word_start == pos {
            break;
        }
        let key = String::from_utf8_lossy(&input[word_start..pos]).into_owned();

        match config_lookup(&entries, &key) {
            Some(val) => println!("{}", val),
            None => println!("NOT_FOUND"),
        }
    }
}
