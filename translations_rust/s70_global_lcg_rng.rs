use std::io::{self, Read};

static mut G_SEED: u32 = 1;

fn next_rand() -> u32 {
    unsafe {
        G_SEED = G_SEED.wrapping_mul(1103515245).wrapping_add(12345);
        (G_SEED >> 16) & 0x7FFF
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let seed: u32 = it.next().unwrap().parse().unwrap();
    let n: i32 = it.next().unwrap().parse().unwrap();
    unsafe {
        G_SEED = seed;
    }
    for _ in 0..n {
        println!("{}", next_rand());
    }
}
