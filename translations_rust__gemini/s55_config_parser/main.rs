use std::io::{self, Read};

const MAX_KEY_LEN: usize = 32;
const MAX_VAL_LEN: usize = 256;
const MAX_ENTRIES: usize = 256;

struct CReader<R: Read> {
    inner: R,
    peeked: Option<u8>,
}

impl<R: Read> CReader<R> {
    fn new(inner: R) -> Self {
        Self { inner, peeked: None }
    }

    fn getchar(&mut self) -> Option<u8> {
        if let Some(b) = self.peeked.take() {
            Some(b)
        } else {
            let mut buf = [0u8; 1];
            if self.inner.read(&mut buf).unwrap_or(0) == 1 {
                Some(buf[0])
            } else {
                None
            }
        }
    }

    fn peekchar(&mut self) -> Option<u8> {
        if self.peeked.is_none() {
            self.peeked = self.getchar();
        }
        self.peeked
    }

    fn scan_int(&mut self) -> Option<i32> {
        while let Some(b) = self.peekchar() {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 11 || b == 12 {
                self.getchar();
            } else {
                break;
            }
        }

        let mut sign = 1;
        if let Some(b) = self.peekchar() {
            if b == b'-' {
                sign = -1;
                self.getchar();
            } else if b == b'+' {
                self.getchar();
            }
        }

        let mut digits = 0;
        let mut val: i32 = 0;
        while let Some(b) = self.peekchar() {
            if b.is_ascii_digit() {
                self.getchar();
                val = val.wrapping_mul(10).wrapping_add((b - b'0') as i32);
                digits += 1;
            } else {
                break;
            }
        }

        if digits == 0 {
            None
        } else {
            Some(val.wrapping_mul(sign))
        }
    }

    fn fgets(&mut self, size: usize) -> Option<Vec<u8>> {
        if size <= 1 {
            return None;
        }
        let max_bytes = size - 1;
        let mut res = Vec::new();

        while res.len() < max_bytes {
            if let Some(b) = self.getchar() {
                res.push(b);
                if b == b'\n' {
                    break;
                }
            } else {
                break;
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    fn scan_31s(&mut self) -> Option<Vec<u8>> {
        while let Some(b) = self.peekchar() {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 11 || b == 12 {
                self.getchar();
            } else {
                break;
            }
        }

        let mut res = Vec::new();
        while res.len() < 31 {
            if let Some(b) = self.peekchar() {
                if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 11 || b == 12 {
                    break;
                }
                self.getchar();
                res.push(b);
            } else {
                break;
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

#[derive(Clone)]
struct ConfigEntry {
    key: Vec<u8>,
    value: Vec<u8>,
}

fn trim(s: &mut Vec<u8>) {
    while let Some(&b) = s.last() {
        if b == b'\n' || b == b'\r' || b == b' ' {
            s.pop();
        } else {
            break;
        }
    }
    let mut start = 0;
    while start < s.len() && s[start] == b' ' {
        start += 1;
    }
    if start > 0 {
        s.drain(0..start);
    }
}

fn parse_config_lines(text: &[u8], max_entries: usize) -> Vec<ConfigEntry> {
    let mut entries = Vec::new();
    let mut p = 0;
    let line_buf_max = MAX_KEY_LEN + MAX_VAL_LEN + 4 - 1;

    while p < text.len() && text[p] != 0 && entries.len() < max_entries {
        let mut line = Vec::new();
        while p < text.len() && text[p] != 0 && text[p] != b'\n' && line.len() < line_buf_max {
            line.push(text[p]);
            p += 1;
        }
        if p < text.len() && text[p] == b'\n' {
            p += 1;
        }

        trim(&mut line);
        if line.is_empty() || line[0] == b'#' {
            continue;
        }

        let eq_pos = match line.iter().position(|&b| b == b'=') {
            Some(pos) => pos,
            None => continue,
        };

        let mut key = line[..eq_pos].to_vec();
        let mut value = line[eq_pos + 1..].to_vec();

        trim(&mut key);
        trim(&mut value);

        if key.len() > MAX_KEY_LEN - 1 {
            key.truncate(MAX_KEY_LEN - 1);
        }
        if value.len() > MAX_VAL_LEN - 1 {
            value.truncate(MAX_VAL_LEN - 1);
        }

        entries.push(ConfigEntry { key, value });
    }

    entries
}

fn config_lookup<'a>(entries: &'a [ConfigEntry], key: &[u8]) -> Option<&'a [u8]> {
    for entry in entries {
        if entry.key == key {
            return Some(&entry.value);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut reader = CReader::new(stdin.lock());

    let n = match reader.scan_int() {
        Some(n) => n,
        None => std::process::exit(1),
    };
    reader.getchar();

    let mut text = Vec::new();
    for _ in 0..n {
        let line = match reader.fgets(256) {
            Some(l) => l,
            None => break,
        };
        let l = line.len();
        if text.len() + l < 4095 {
            text.extend_from_slice(&line);
        }
    }
    text.push(0);

    let entries = parse_config_lines(&text, MAX_ENTRIES);

    let m = match reader.scan_int() {
        Some(m) => m,
        None => std::process::exit(1),
    };

    for _ in 0..m {
        let key = match reader.scan_31s() {
            Some(k) => k,
            None => break,
        };
        let val = config_lookup(&entries, &key);
        if let Some(v) = val {
            println!("{}", String::from_utf8_lossy(v));
        } else {
            println!("NOT_FOUND");
        }
    }
}
