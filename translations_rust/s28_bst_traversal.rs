use std::io::{self, Read};

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node { value: v, left: None, right: None })),
        Some(mut n) => {
            if v < n.value {
                n.left = insert(n.left, v);
            } else {
                n.right = insert(n.right, v);
            }
            Some(n)
        }
    }
}

fn count_nodes(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(n) => 1 + count_nodes(&n.left) + count_nodes(&n.right),
    }
}

fn height(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let lh = height(&n.left);
            let rh = height(&n.right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

fn inorder(root: &Option<Box<Node>>, out: &mut Vec<String>) {
    if let Some(n) = root {
        inorder(&n.left, out);
        out.push(n.value.to_string());
        inorder(&n.right, out);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|t| t.parse::<i32>().unwrap());

    let n = it.next().unwrap();
    let mut root: Option<Box<Node>> = None;
    for _ in 0..n {
        let v = it.next().unwrap();
        root = insert(root, v);
    }

    let mut out: Vec<String> = Vec::new();
    inorder(&root, &mut out);
    println!("{}", out.join(" "));
    println!("count={} height={}", count_nodes(&root), height(&root));
}
