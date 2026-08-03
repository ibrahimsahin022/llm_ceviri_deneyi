use std::io::{self, Read};

type BinOp = fn(i32, i32) -> i32;

fn op_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

fn op_sub(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

fn op_mul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

struct OpEntry {
    name: &'static str,
    op_fn: BinOp,
}

static DISPATCH_TABLE: &[OpEntry] = &[
    OpEntry { name: "add", op_fn: op_add },
    OpEntry { name: "sub", op_fn: op_sub },
    OpEntry { name: "mul", op_fn: op_mul },
];

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };
        let a: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        let b: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };

        if op == "div" {
            if b == 0 {
                println!("DIVZERO");
            } else {
                println!("{}", a.wrapping_div(b));
            }
            continue;
        }

        let mut found = false;
        for entry in DISPATCH_TABLE {
            if op == entry.name {
                println!("{}", (entry.op_fn)(a, b));
                found = true;
                break;
            }
        }

        if !found {
            println!("UNKNOWN");
        }
    }
}
