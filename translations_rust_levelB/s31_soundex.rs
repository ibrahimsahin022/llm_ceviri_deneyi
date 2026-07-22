use std::io::{self, Read};

fn add_code(code: &mut [i8; 128], s: &str, c: i8) {
    for ch in s.chars() {
        let idx = ch as usize;
        code[idx] = c;
        code[idx ^ 0x20] = c;
    }
}

fn init() -> [i8; 128] {
    let mut code = [0i8; 128];
    let classes = ["AEIOU", "", "BFPV", "CGJKQSXZ", "DT", "L", "MN", "R"];
    for (i, cls) in classes.iter().enumerate() {
        add_code(&mut code, cls, i as i8 - 1);
    }
    code
}

fn soundex(code: &[i8; 128], s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return "0000".to_string();
    }

    let mut out: Vec<char> = vec![chars[0]];
    let mut prev = code[chars[0] as usize];

    let mut idx = 1;
    while idx < chars.len() && out.len() < 4 {
        let c = code[chars[idx] as usize];
        if c != prev {
            if c == -1 {
                prev = 0;
            } else if c > 0 {
                out.push((b'0' + c as u8) as char);
                prev = c;
            }
        }
        idx += 1;
    }
    while out.len() < 4 {
        out.push('0');
    }
    out.into_iter().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let name = input.lines().next().unwrap_or("").trim();

    let code = init();
    println!("{}", soundex(&code, name));
}
