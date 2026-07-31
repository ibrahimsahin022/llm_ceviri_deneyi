use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

const COMMAND_NAMES: &[(&str, Command)] = &[
    ("add", Command::Add),
    ("sub", Command::Sub),
    ("mul", Command::Mul),
    ("div", Command::Div),
];

fn find_command(name: &str) -> Option<Command> {
    for &(cmd_name, cmd) in COMMAND_NAMES {
        if cmd_name == name {
            return Some(cmd);
        }
    }
    None
}

fn apply_command(cmd: Command, a: i32, b: i32) -> i32 {
    match cmd {
        Command::Add => a + b,
        Command::Sub => a - b,
        Command::Mul => a * b,
        Command::Div => if b != 0 { a / b } else { 0 },
    }
}

fn command_name(cmd: Command) -> &'static str {
    match cmd {
        Command::Add => "add",
        Command::Sub => "sub",
        Command::Mul => "mul",
        Command::Div => "div",
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            if let (Ok(a), Ok(b)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                let cmd_name = parts[0];
                match find_command(cmd_name) {
                    None => {
                        println!("UNKNOWN");
                        return;
                    }
                    Some(cmd) => {
                        let result = apply_command(cmd, a, b);
                        println!("{}({},{})={}", command_name(cmd), a, b, result);

                        let mut x = a;
                        let m = std::cmp::max(x, 10);
                        x += 1;
                        println!("m={} x={}", m, x);
                    }
                }
            }
        }
    }
}
