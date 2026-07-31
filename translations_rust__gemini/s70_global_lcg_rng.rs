use std::io::{self, Read};

fn next_rand(g_seed: &mut u32) -> u32 {
    *g_seed = g_seed.wrapping_mul(1103515245).wrapping_add(12345);
    (*g_seed >> 16) & 0x7FFF
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut words = input.split_whitespace();

    let seed: u32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let n: i32 = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut g_seed = seed;
    for _ in 0..n {
        println!("{}", next_rand(&mut g_seed));
    }
}
