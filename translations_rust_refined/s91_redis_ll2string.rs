use std::io::{self, BufRead};

fn ull2string(value: u64) -> String {
    value.to_string()
}

fn ll2string(svalue: i64) -> String {
    if svalue < 0 {
        let value: u64 = if svalue != i64::MIN {
            (-svalue) as u64
        } else {
            (i64::MAX as u64) + 1
        };
        format!("-{}", ull2string(value))
    } else {
        ull2string(svalue as u64)
    }
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let v: i64 = line.trim().parse().unwrap();
    println!("{}", ll2string(v));
}
