use std::io::{self, Read};

macro_rules! define_commands {
    ($(($variant:ident, $str:expr)),* $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[repr(i32)]
        enum Command {
            $($variant),*
        }

        const COMMAND_NAMES: &[&str] = &[
            $($str),*
        ];

        impl Command {
            fn from_idx(idx: usize) -> Option<Self> {
                const ALL: &[Command] = &[ $(Command::$variant),* ];
                ALL.get(idx).copied()
            }
        }
    };
}

define_commands!(
    (ADD, "add"),
    (SUB, "sub"),
    (MUL, "mul"),
    (DIV, "div"),
);

fn find_command(name: &str) -> i32 {
    for (i, &cmd_name) in COMMAND_NAMES.iter().enumerate() {
        if cmd_name == name {
            return i as i32;
        }
    }
    -1
}

fn apply_command(cmd: Command, a: i32, b: i32) -> i32 {
    match cmd {
        Command::ADD => a.wrapping_add(b),
        Command::SUB => a.wrapping_sub(b),
        Command::MUL => a.wrapping_mul(b),
        Command::DIV => {
            if b != 0 {
                a.wrapping_div(b)
            } else {
                0
            }
        }
    }
}

macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
}

fn post_inc(x: &mut i32) -> i32 {
    let val = *x;
    *x = x.wrapping_add(1);
    val
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut words = input.split_whitespace();

    let cmd_name = match words.next() {
        Some(s) => {
            if s.len() > 15 {
                &s[..15]
            } else {
                s
            }
        }
        None => std::process::exit(1),
    };

    let a: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let b: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let idx = find_command(cmd_name);
    if idx < 0 {
        println!("UNKNOWN");
        return;
    }

    let cmd = Command::from_idx(idx as usize).unwrap();
    let result = apply_command(cmd, a, b);
    println!("{}({},{})={}", COMMAND_NAMES[idx as usize], a, b, result);

    let mut x = a;
    let m = max!(post_inc(&mut x), 10);
    println!("m={} x={}", m, x);
}
