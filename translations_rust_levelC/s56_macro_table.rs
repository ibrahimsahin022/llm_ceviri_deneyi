use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

const COMMAND_NAMES: [&str; 4] = ["add", "sub", "mul", "div"];

fn find_command(name: &str) -> Option<Command> {
    match name {
        "add" => Some(Command::Add),
        "sub" => Some(Command::Sub),
        "mul" => Some(Command::Mul),
        "div" => Some(Command::Div),
        _ => None,
    }
}

fn apply_command(cmd: Command, a: i32, b: i32) -> i32 {
    match cmd {
        Command::Add => a + b,
        Command::Sub => a - b,
        Command::Mul => a * b,
        Command::Div => if b != 0 { a / b } else { 0 },
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let cmd_name = it.next().unwrap();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    let cmd = match find_command(cmd_name) {
        Some(c) => c,
        None => {
            println!("UNKNOWN");
            return;
        }
    };
    let idx = match cmd_name {
        "add" => 0,
        "sub" => 1,
        "mul" => 2,
        _ => 3,
    };
    let result = apply_command(cmd, a, b);
    println!("{}({},{})={}", COMMAND_NAMES[idx], a, b, result);

    let mut x = a;
    let m = max(x, 10);
    x += 1;
    println!("m={} x={}", m, x);
}
