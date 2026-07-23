use std::io::{self, Read};

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(node: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match node {
        None => Some(Box::new(Node {
            value: v,
            left: None,
            right: None,
        })),
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

fn count_nodes(node: &Option<Box<Node>>) -> i32 {
    match node {
        None => 0,
        Some(n) => 1 + count_nodes(&n.left) + count_nodes(&n.right),
    }
}

fn height(node: &Option<Box<Node>>) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let lh = height(&n.left);
            let rh = height(&n.right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

fn inorder(node: &Option<Box<Node>>, first_print: &mut bool) {
    if let Some(n) = node {
        inorder(&n.left, first_print);
        if !*first_print {
            print!(" ");
        }
        print!("{}", n.value);
        *first_print = false;
        inorder(&n.right, first_print);
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut root: Option<Box<Node>> = None;
    for _ in 0..n {
        if let Some(v_str) = tokens.next() {
            if let Ok(v) = v_str.parse::<i32>() {
                root = insert(root, v);
            }
        }
    }

    let mut first_print = true;
    inorder(&root, &mut first_print);
    println!();
    println!("count={} height={}", count_nodes(&root), height(&root));
}
