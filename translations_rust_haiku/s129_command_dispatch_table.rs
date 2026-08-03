use std::io;

fn cmd_set(state: &mut i32, arg: i32) {
    *state = arg;
}

fn cmd_add(state: &mut i32, arg: i32) {
    *state += arg;
}

fn cmd_sub(state: &mut i32, arg: i32) {
    *state -= arg;
}

fn cmd_mul(state: &mut i32, arg: i32) {
    *state *= arg;
}

type CmdFn = fn(&mut i32, i32);

struct CmdEntry {
    name: &'static str,
    fn_ptr: CmdFn,
}

const TABLE: &[CmdEntry] = &[
    CmdEntry { name: "SET", fn_ptr: cmd_set },
    CmdEntry { name: "ADD", fn_ptr: cmd_add },
    CmdEntry { name: "SUB", fn_ptr: cmd_sub },
    CmdEntry { name: "MUL", fn_ptr: cmd_mul },
];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let n: usize = input.trim().parse().unwrap_or(0);

    let mut state = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let cmd = parts[0];

        if cmd == "PRINT" {
            println!("{}", state);
            continue;
        }

        let arg: i32 = if parts.len() > 1 {
            parts[1].parse().unwrap_or(0)
        } else {
            0
        };

        for entry in TABLE {
            if entry.name == cmd {
                (entry.fn_ptr)(&mut state, arg);
                break;
            }
        }
    }
}
