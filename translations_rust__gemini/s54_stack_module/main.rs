use std::io::{self, Read};

const STACK_CAPACITY: usize = 100;

struct Stack {
    data: [i32; STACK_CAPACITY],
    top: i32,
}

fn stack_init(s: &mut Stack) {
    s.top = -1;
}

fn stack_is_empty(s: &Stack) -> bool {
    s.top < 0
}

fn stack_is_full(s: &Stack) -> bool {
    s.top >= (STACK_CAPACITY as i32) - 1
}

fn stack_push(s: &mut Stack, value: i32) -> i32 {
    if stack_is_full(s) {
        return 0;
    }
    s.top += 1;
    s.data[s.top as usize] = value;
    1
}

fn stack_pop(s: &mut Stack, out: &mut i32) -> i32 {
    if stack_is_empty(s) {
        return 0;
    }
    *out = s.data[s.top as usize];
    s.top -= 1;
    1
}

fn stack_peek(s: &Stack, out: &mut i32) -> i32 {
    if stack_is_empty(s) {
        return 0;
    }
    *out = s.data[s.top as usize];
    1
}

fn stack_size(s: &Stack) -> i32 {
    s.top + 1
}

fn main() {
    let mut s = Stack {
        data: [0; STACK_CAPACITY],
        top: -1,
    };
    stack_init(&mut s);

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next() {
        Some(tok) => match tok.parse() {
            Ok(val) => val,
            Err(_) => return,
        },
        None => return,
    };

    for _ in 0..n {
        let cmd = match tokens.next() {
            Some(tok) => tok,
            None => break,
        };

        if cmd == "PUSH" {
            let v: i32 = match tokens.next() {
                Some(tok) => match tok.parse() {
                    Ok(val) => val,
                    Err(_) => break,
                },
                None => break,
            };
            let ok = stack_push(&mut s, v);
            if ok != 0 {
                println!("OK");
            } else {
                println!("FULL");
            }
        } else if cmd == "POP" {
            let mut v = 0;
            if stack_pop(&mut s, &mut v) != 0 {
                println!("{}", v);
            } else {
                println!("EMPTY");
            }
        } else if cmd == "PEEK" {
            let mut v = 0;
            if stack_peek(&s, &mut v) != 0 {
                println!("{}", v);
            } else {
                println!("EMPTY");
            }
        } else if cmd == "SIZE" {
            println!("{}", stack_size(&s));
        }
    }
}
