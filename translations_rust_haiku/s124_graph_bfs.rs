use std::io;

const MAX_V: usize = 100;

fn main() {
    let mut adj: Vec<Vec<usize>> = vec![vec![]; MAX_V];
    let mut visited = vec![false; MAX_V];
    let mut queue = vec![0; MAX_V];

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let v: usize = parts[0].parse().unwrap_or(0);
    let e: usize = parts[1].parse().unwrap_or(0);

    for _ in 0..e {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let a: usize = parts[0].parse().unwrap_or(0);
        let b: usize = parts[1].parse().unwrap_or(0);
        adj[a].push(b);
        adj[b].push(a);
    }

    input.clear();
    io::stdin().read_line(&mut input).ok();
    let start: usize = input.trim().parse().unwrap_or(0);

    let mut qhead = 0;
    let mut qtail = 0;
    queue[qtail] = start;
    qtail += 1;
    visited[start] = true;

    let mut first = true;
    while qhead < qtail {
        let cur = queue[qhead];
        qhead += 1;
        if !first {
            print!(" ");
        }
        print!("{}", cur);
        first = false;
        for &next in &adj[cur] {
            if !visited[next] {
                visited[next] = true;
                queue[qtail] = next;
                qtail += 1;
            }
        }
    }
    println!();
}
