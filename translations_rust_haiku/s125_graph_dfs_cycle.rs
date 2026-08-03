use std::io;

const MAX_V: usize = 100;

fn dfs(u: usize, adj: &Vec<Vec<usize>>, color: &mut Vec<i32>, has_cycle: &mut bool) {
    color[u] = 1;
    for &v in &adj[u] {
        if color[v] == 1 {
            *has_cycle = true;
            return;
        }
        if color[v] == 0 {
            dfs(v, adj, color, has_cycle);
            if *has_cycle {
                return;
            }
        }
    }
    color[u] = 2;
}

fn main() {
    let mut adj: Vec<Vec<usize>> = vec![vec![]; MAX_V];
    let mut color = vec![0; MAX_V]; // 0=white, 1=gray, 2=black
    let mut has_cycle = false;

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
    }

    for i in 0..v {
        if !has_cycle && color[i] == 0 {
            dfs(i, &adj, &mut color, &mut has_cycle);
        }
    }

    println!("{}", if has_cycle { "CYCLE" } else { "NOCYCLE" });
}
