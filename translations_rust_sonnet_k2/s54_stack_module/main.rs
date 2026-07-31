use std::io::{self, Read};

const STACK_CAPACITY: usize = 128;

struct Stack {
    data: [i32; STACK_CAPACITY],
    top: i32,
}

impl Stack {
    fn new() -> Self {
        Stack {
            data: [0; STACK_CAPACITY],
            top: -1,
        }
    }

    fn is_empty(&self) -> bool {
        self.top < 0
    }

    fn is_full(&self) -> bool {
        self.top >= (STACK_CAPACITY as i32) - 1
    }

    fn push(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.top += 1;
        self.data[self.top as usize] = value;
        true
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        let v = self.data[self.top as usize];
        self.top -= 1;
        Some(v)
    }

    fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        Some(self.data[self.top as usize])
    }

    fn size(&self) -> i32 {
        self.top + 1
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

    let mut s = Stack::new();

    // scanf("%d", &n)
    while pos < len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let n_start = pos;
    if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let n_digits_start = pos;
    while pos < len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if n_digits_start == pos {
        return;
    }
    let n: i32 = match std::str::from_utf8(&input[n_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    for _ in 0..n {
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
            break;
        }
        let cmd = String::from_utf8_lossy(&input[word_start..pos]).into_owned();

        if cmd == "PUSH" {
            while pos < len && is_space_ws(input[pos]) {
                pos += 1;
            }
            let v_start = pos;
            if pos < len && (input[pos] == b'+' || input[pos] == b'-') {
                pos += 1;
            }
            let v_digits_start = pos;
            while pos < len && input[pos].is_ascii_digit() {
                pos += 1;
            }
            if v_digits_start == pos {
                break;
            }
            let v: i32 = match std::str::from_utf8(&input[v_start..pos])
                .ok()
                .and_then(|s| s.parse().ok())
            {
                Some(v) => v,
                None => break,
            };
            let ok = s.push(v);
            println!("{}", if ok { "OK" } else { "FULL" });
        } else if cmd == "POP" {
            match s.pop() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            }
        } else if cmd == "PEEK" {
            match s.peek() {
                Some(v) => println!("{}", v),
                None => println!("EMPTY"),
            }
        } else if cmd == "SIZE" {
            println!("{}", s.size());
        }
    }
}
