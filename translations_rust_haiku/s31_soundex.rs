use std::io::{self, BufRead};

fn add_code(s: &str, c: i32, code: &mut [i32; 128]) {
    for ch in s.chars() {
        let idx = ch as usize;
        if idx < 128 {
            code[idx] = c;
            code[(0x20 ^ idx)] = c;
        }
    }
}

fn init(code: &mut [i32; 128]) {
    let cls = vec!["AEIOU", "", "BFPV", "CGJKQSXZ", "DT", "L", "MN", "R"];
    for (i, s) in cls.iter().enumerate() {
        add_code(s, (i as i32) - 1, code);
    }
}

fn soundex(s: &str, code: &[i32; 128]) -> String {
    let mut out = String::from("0000");
    if s.is_empty() {
        return out;
    }

    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        out.clear();
        out.push(first);

        let mut prev = code[first as usize];
        let mut i = 1;

        for c in chars {
            if i >= 4 {
                break;
            }
            let code_val = code[c as usize];
            if code_val == prev {
                continue;
            }

            if code_val == -1 {
                prev = 0;
            } else if code_val > 0 {
                out.push((code_val as u8 + b'0') as char);
                i += 1;
                prev = code_val;
            }
        }

        while out.len() < 4 {
            out.push('0');
        }
    }

    out
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut line)) = lines.next() {
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }

        let mut code = [0i32; 128];
        init(&mut code);

        println!("{}", soundex(&line, &code));
    }
}
