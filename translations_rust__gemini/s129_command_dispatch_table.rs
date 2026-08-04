use std::io::{self, Read};

type CmdFn = fn(&mut i32, i32);

fn cmd_set(state: &mut i32, arg: i32) {
    *state = arg;
}

fn cmd_add(state: &mut i32, arg: i32) {
    *state = state.wrapping_add(arg);
}

fn cmd_sub(state: &mut i32, arg: i32) {
    *state = state.wrapping_sub(arg);
}

fn cmd_mul(state: &mut i32, arg: i32) {
    *state = state.wrapping_mul(arg);
}

struct CmdEntry {
    name: &'static str,
    fn_ptr: CmdFn,
}

static TABLE: [CmdEntry; 4] = [
    CmdEntry { name: "SET", fn_ptr: cmd_set },
    CmdEntry { name: "ADD", fn_ptr: cmd_add },
    CmdEntry { name: "SUB", fn_ptr: cmd_sub },
    CmdEntry { name: "MUL", fn_ptr: cmd_mul },
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

    let mut state: i32 = 0;

    for _ in 0..n {
        let cmd = match tokens.next() {
            Some(s) => s,
            None => return,
        };

        if cmd == "PRINT" {
            println!("{}", state);
            continue;
        }

        let arg: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };

        for entry in TABLE.iter() {
            if cmd == entry.name {
                (entry.fn_ptr)(&mut state, arg);
                break;
            }
        }
    }
}
