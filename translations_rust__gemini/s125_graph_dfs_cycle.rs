use std::io::{self, Read};

fn dfs(u: usize, adj: &[Vec<usize>], color: &mut [u8], has_cycle: &mut bool) {
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
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let v: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let e: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut adj = vec![Vec::new(); v];
    for _ in 0..e {
        let a: usize = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        let b: usize = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(val) => val,
            None => return,
        };
        if a < v {
            adj[a].push(b);
        }
    }

    let mut color = vec![0u8; v];
    let mut has_cycle = false;

    for i in 0..v {
        if has_cycle {
            break;
        }
        if color[i] == 0 {
            dfs(i, &adj, &mut color, &mut has_cycle);
        }
    }

    if has_cycle {
        println!("CYCLE");
    } else {
        println!("NOCYCLE");
    }
}
