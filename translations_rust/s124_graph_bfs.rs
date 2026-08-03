use std::collections::VecDeque;
use std::io::{self, Read};

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
        adj[b].push(a);
    }

    let start: usize = it.next().unwrap().parse().unwrap();

    let mut visited = vec![false; v];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start] = true;

    let mut result = Vec::new();
    while let Some(cur) = queue.pop_front() {
        result.push(cur.to_string());
        for &next in &adj[cur] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    println!("{}", result.join(" "));
}
