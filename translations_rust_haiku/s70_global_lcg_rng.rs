use std::io;

static mut G_SEED: u32 = 1;

fn next_rand() -> u32 {
    unsafe {
        G_SEED = G_SEED.wrapping_mul(1103515245u32).wrapping_add(12345u32);
        (G_SEED >> 16) & 0x7FFFu32
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return;
    }

    let seed: u32 = match parts[0].parse() {
        Ok(s) => s,
        Err(_) => return,
    };

    let n: usize = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => return,
    };

    unsafe {
        G_SEED = seed;
    }

    for _ in 0..n {
        println!("{}", next_rand());
    }
}
