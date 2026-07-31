use std::cell::RefCell;
use std::io::{self, Read};

thread_local! {
    static ERRBUF: RefCell<String> = RefCell::new(String::new());
}

fn set_error(msg: &str) {
    ERRBUF.with(|buf| {
        let mut b = buf.borrow_mut();
        b.clear();
        b.push_str(msg);
    });
}

fn get_error() -> String {
    ERRBUF.with(|buf| buf.borrow().clone())
}

fn safe_divide(a: i32, b: i32, out: &mut i32) -> i32 {
    if b == 0 {
        set_error("division by zero");
        return -1;
    }
    *out = a.wrapping_div(b);
    0
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();
    while let Some(a_str) = tokens.next() {
        let Ok(a) = a_str.parse::<i32>() else {
            break;
        };
        let Some(b_str) = tokens.next() else {
            break;
        };
        let Ok(b) = b_str.parse::<i32>() else {
            break;
        };

        let mut r = 0;
        if safe_divide(a, b, &mut r) == 0 {
            println!("{}", r);
        } else {
            println!("HATA: {}", get_error());
        }
    }
}
