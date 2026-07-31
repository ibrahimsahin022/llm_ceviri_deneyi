use std::io::{self, BufRead};

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

fn stack_push(s: &mut Stack, value: i32) -> bool {
    if stack_is_full(s) {
        return false;
    }
    s.top += 1;
    s.data[s.top as usize] = value;
    true
}

fn stack_pop(s: &mut Stack) -> Option<i32> {
    if stack_is_empty(s) {
        return None;
    }
    let val = s.data[s.top as usize];
    s.top -= 1;
    Some(val)
}

fn stack_peek(s: &Stack) -> Option<i32> {
    if stack_is_empty(s) {
        return None;
    }
    Some(s.data[s.top as usize])
}

fn stack_size(s: &Stack) -> i32 {
    s.top + 1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut s = Stack {
        data: [0; STACK_CAPACITY],
        top: -1,
    };
    stack_init(&mut s);

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let cmd = parts[0];
            match cmd {
                "PUSH" => {
                    if parts.len() >= 2 {
                        if let Ok(v) = parts[1].parse::<i32>() {
                            if stack_push(&mut s, v) {
                                println!("OK");
                            } else {
                                println!("FULL");
                            }
                        }
                    }
                }
                "POP" => {
                    if let Some(v) = stack_pop(&mut s) {
                        println!("{}", v);
                    } else {
                        println!("EMPTY");
                    }
                }
                "PEEK" => {
                    if let Some(v) = stack_peek(&s) {
                        println!("{}", v);
                    } else {
                        println!("EMPTY");
                    }
                }
                "SIZE" => {
                    println!("{}", stack_size(&s));
                }
                _ => {}
            }
        }
    }
}
