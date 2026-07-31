use std::io::{self, BufRead};

struct Sds {
    content: String,
}

impl Sds {
    fn new(init: &str) -> Self {
        Sds {
            content: init.to_string(),
        }
    }

    fn empty() -> Self {
        Sds {
            content: String::new(),
        }
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn cat(&mut self, t: &str) {
        self.content.push_str(t);
    }

    fn trim(&mut self, cset: &str) {
        let mut start = 0;
        let mut end = self.content.len();

        for (i, c) in self.content.chars().enumerate() {
            if !cset.contains(c) {
                start = i;
                break;
            }
        }

        for (i, c) in self.content.chars().rev().enumerate() {
            if !cset.contains(c) {
                end = self.content.len() - i;
                break;
            }
        }

        if start < end {
            self.content = self.content[start..end].to_string();
        } else {
            self.content.clear();
        }
    }

    fn range(&mut self, start: i32, end: i32) {
        let len = self.content.len() as i32;
        let mut s = start;
        let mut e = end;

        if len == 0 {
            return;
        }

        if s < 0 {
            s = len + s;
        }
        if e < 0 {
            e = len + e;
        }

        if s < 0 {
            s = 0;
        }
        if e >= len {
            e = len - 1;
        }

        if s > e {
            self.content.clear();
        } else {
            let newlen = (e - s + 1) as usize;
            self.content = self.content[s as usize..(s as usize + newlen)].to_string();
        }
    }

    fn tolower(&mut self) {
        self.content = self.content.to_lowercase();
    }

    fn toupper(&mut self) {
        self.content = self.content.to_uppercase();
    }

    fn cmp(&self, other: &Sds) -> i32 {
        match self.content.cmp(&other.content) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => {
                if self.content.len() > other.content.len() {
                    1
                } else if self.content.len() < other.content.len() {
                    -1
                } else {
                    0
                }
            }
            std::cmp::Ordering::Greater => 1,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let ncmd: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut cur = Sds::empty();

    for _ in 0..ncmd {
        if let Some(Ok(line)) = lines.next() {
            let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
            if let Some(sp_idx) = trimmed.find(' ') {
                let cmd = &trimmed[..sp_idx];
                let arg = &trimmed[sp_idx + 1..];

                match cmd {
                    "NEW" => {
                        cur = Sds::new(arg);
                    }
                    "CAT" => {
                        cur.cat(arg);
                    }
                    "TRIM" => {
                        cur.trim(arg);
                    }
                    "RANGE" => {
                        let parts: Vec<&str> = arg.split_whitespace().collect();
                        if parts.len() == 2 {
                            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                                cur.range(a, b);
                            }
                        }
                    }
                    "LOWER" => {
                        cur.tolower();
                    }
                    "UPPER" => {
                        cur.toupper();
                    }
                    "CMP" => {
                        let tmp = Sds::new(arg);
                        println!("CMP={}", cur.cmp(&tmp));
                        continue;
                    }
                    _ => {}
                }
            } else {
                match trimmed {
                    "LOWER" => cur.tolower(),
                    "UPPER" => cur.toupper(),
                    _ => {}
                }
            }
            println!("LEN={} STR={}", cur.len(), cur.content);
        }
    }
}
