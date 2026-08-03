use std::io::{self, BufRead};

fn ull2string(value: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let mut tmp = String::new();
    let mut val = value;
    while val > 0 {
        tmp.push((b'0' + (val % 10) as u8) as char);
        val /= 10;
    }

    tmp.chars().rev().collect()
}

fn ll2string(svalue: i64) -> String {
    let (value, negative) = if svalue < 0 {
        if svalue != i64::MIN {
            ((-svalue) as u64, true)
        } else {
            ((i64::MAX as u64) + 1, true)
        }
    } else {
        (svalue as u64, false)
    };

    let mut result = String::new();
    if negative {
        result.push('-');
    }
    result.push_str(&ull2string(value));
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(v) = line.trim().parse::<i64>() {
            println!("{}", ll2string(v));
        }
    }
}
