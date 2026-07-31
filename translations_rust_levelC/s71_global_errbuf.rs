use std::io::{self, Read};

static mut G_ERRBUF: String = String::new();

fn set_error(msg: &str) {
    unsafe {
        G_ERRBUF = msg.to_string();
    }
}

fn get_error() -> String {
    unsafe { G_ERRBUF.clone() }
}

fn safe_divide(a: i32, b: i32) -> Result<i32, ()> {
    if b == 0 {
        set_error("division by zero");
        return Err(());
    }
    Ok(a / b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    loop {
        let a: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        let b: i32 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break,
        };
        match safe_divide(a, b) {
            Ok(r) => println!("{}", r),
            Err(_) => println!("HATA: {}", get_error()),
        }
    }
}
