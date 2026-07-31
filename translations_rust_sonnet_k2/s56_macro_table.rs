use std::io::{self, Read};

/* X-Macro benzeri tek-kaynak komut listesi */
const COMMAND_LIST: [(&str, &str); 4] = [
    ("ADD", "add"),
    ("SUB", "sub"),
    ("MUL", "mul"),
    ("DIV", "div"),
];

fn find_command(name: &str) -> i32 {
    for (i, &(_, s)) in COMMAND_LIST.iter().enumerate() {
        if s == name {
            return i as i32;
        }
    }
    -1
}

fn apply_command(idx: usize, a: i32, b: i32) -> i32 {
    match idx {
        0 => a + b,
        1 => a - b,
        2 => a * b,
        3 => {
            if b != 0 {
                a / b
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let len = input.len();
    let mut pos = 0usize;

    // %15s
    while pos < len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let word_start = pos;
    let mut wcount = 0usize;
    while pos < len && !is_space_ws(input[pos]) && wcount < 15 {
        pos += 1;
        wcount += 1;
    }
    if word_start == pos {
        return;
    }
    let cmd_name = String::from_utf8_lossy(&input[word_start..pos]).into_owned();

    // %d
    while pos < len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let a_start = pos;
    if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let a_digits_start = pos;
    while pos < len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if a_digits_start == pos {
        return;
    }
    let a: i32 = match std::str::from_utf8(&input[a_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    // %d
    while pos < len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let b_start = pos;
    if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let b_digits_start = pos;
    while pos < len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if b_digits_start == pos {
        return;
    }
    let b: i32 = match std::str::from_utf8(&input[b_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    let idx = find_command(&cmd_name);
    if idx < 0 {
        println!("UNKNOWN");
        return;
    }
    let result = apply_command(idx as usize, a, b);
    println!(
        "{}({},{})={}",
        COMMAND_LIST[idx as usize].1,
        a,
        b,
        result
    );

    /* Coklu-degerlendirme tuzagi: MAX(x++, 10) makrosunun C'deki tam
     * genisletilmis davranisi elle simule edilir: govde icinde 'a' (x++)
     * parametresi IKI KEZ gecer (kosulda ve secilirse true-dalinda). */
    let mut x = a;
    let cond_val = x;
    x += 1;
    let m;
    if cond_val > 10 {
        m = x;
        x += 1;
    } else {
        m = 10;
    }

    println!("m={} x={}", m, x);
}
