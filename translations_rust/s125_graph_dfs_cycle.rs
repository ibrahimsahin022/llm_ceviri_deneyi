use std::io::{self, Read};

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); v];
    for _ in 0..e {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        adj[a].push(b);
    }

    let mut color = vec![0; v];
    let mut has_cycle = false;

    for i in 0..v {
        if has_cycle {
            break;
        }
        if color[i] == 0 {
            dfs(i, &adj, &mut color, &mut has_cycle);
        }
    }

    println!("{}", if has_cycle { "CYCLE" } else { "NOCYCLE" });
}
