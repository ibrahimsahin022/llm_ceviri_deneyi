use std::io::{self, Read};

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node {
            value: v,
            left: None,
            right: None,
        })),
        Some(mut node) => {
            if v < node.value {
                node.left = insert(node.left.take(), v);
            } else {
                node.right = insert(node.right.take(), v);
            }
            Some(node)
        }
    }
}

fn count_nodes(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(node) => 1 + count_nodes(&node.left) + count_nodes(&node.right),
    }
}

fn height(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let lh = height(&node.left);
            let rh = height(&node.right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

fn inorder(root: &Option<Box<Node>>, first_print: &mut bool, out: &mut String) {
    if let Some(node) = root {
        inorder(&node.left, first_print, out);
        if !*first_print {
            out.push(' ');
        }
        out.push_str(&node.value.to_string());
        *first_print = false;
        inorder(&node.right, first_print, out);
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut root: Option<Box<Node>> = None;
    for _ in 0..n {
        let v: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        root = insert(root, v);
    }

    let mut first_print = true;
    let mut out = String::new();
    inorder(&root, &mut first_print, &mut out);
    println!("{}", out);
    println!("count={} height={}", count_nodes(&root), height(&root));
}
