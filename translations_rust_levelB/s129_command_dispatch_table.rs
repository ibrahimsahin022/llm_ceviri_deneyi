use std::io::{self, Read};

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

fn main() {
    let table: [(&str, fn(&mut i32, i32)); 4] = [
        ("SET", cmd_set),
        ("ADD", cmd_add),
        ("SUB", cmd_sub),
        ("MUL", cmd_mul),
    ];

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut state: i32 = 0;

    for _ in 0..n {
        let cmd = it.next().unwrap();
        if cmd == "PRINT" {
            println!("{}", state);
            continue;
        }
        let arg: i32 = it.next().unwrap().parse().unwrap();
        for (name, f) in table.iter() {
            if *name == cmd {
                f(&mut state, arg);
                break;
            }
        }
    }
}
