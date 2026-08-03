use std::io::{self, Read};

const MAX_V: usize = 100;
const MAX_DEG: usize = 20;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let _v: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };
    let e: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut adj = [[0usize; MAX_DEG]; MAX_V];
    let mut adj_count = [0usize; MAX_V];

    for _ in 0..e {
        let a: usize = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        let b: usize = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };

        adj[a][adj_count[a]] = b;
        adj_count[a] += 1;
        adj[b][adj_count[b]] = a;
        adj_count[b] += 1;
    }

    let start: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut visited = [0i32; MAX_V];
    let mut queue = [0usize; MAX_V];
    let mut qhead = 0;
    let mut qtail = 0;

    queue[qtail] = start;
    qtail += 1;
    visited[start] = 1;

    let mut first = true;
    while qhead < qtail {
        let cur = queue[qhead];
        qhead += 1;

        if !first {
            print!(" ");
        }
        print!("{}", cur);
        first = false;

        for i in 0..adj_count[cur] {
            let next = adj[cur][i];
            if visited[next] == 0 {
                visited[next] = 1;
                queue[qtail] = next;
                qtail += 1;
            }
        }
    }
    println!();
}
