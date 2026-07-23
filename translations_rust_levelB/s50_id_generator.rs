use std::io::{self, Read};

const BASLANGIC_ID: i32 = 1000;

static mut COUNTER: i32 = BASLANGIC_ID;

fn next_id() -> i32 {
    unsafe {
        let id = COUNTER;
        COUNTER += 1;
        id
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = match input.trim().parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    for _ in 0..n {
        println!("{}", next_id());
    }
}
