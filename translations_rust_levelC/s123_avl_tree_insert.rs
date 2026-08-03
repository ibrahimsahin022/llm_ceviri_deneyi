use std::io::{self, Read};

struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn node_height(n: &Option<Box<Node>>) -> i32 {
    match n {
        Some(node) => node.height,
        None => 0,
    }
}

fn new_node(value: i32) -> Box<Node> {
    Box::new(Node {
        value,
        height: 1,
        left: None,
        right: None,
    })
}

fn rotate_right(mut y: Box<Node>) -> Box<Node> {
    let mut x = y.left.take().unwrap();
    let t2 = x.right.take();
    y.left = t2;
    y.height = std::cmp::max(node_height(&y.left), node_height(&y.right)) + 1;
    x.right = Some(y);
    x.height = std::cmp::max(node_height(&x.left), node_height(&x.right)) + 1;
    x
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.right.take().unwrap();
    let t2 = y.left.take();
    x.right = t2;
    x.height = std::cmp::max(node_height(&x.left), node_height(&x.right)) + 1;
    y.left = Some(x);
    y.height = std::cmp::max(node_height(&y.left), node_height(&y.right)) + 1;
    y
}

fn balance_factor(n: &Node) -> i32 {
    node_height(&n.left) - node_height(&n.right)
}

fn avl_insert(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    let mut node = match node {
        None => return Some(new_node(value)),
        Some(n) => n,
    };

    if value < node.value {
        node.left = avl_insert(node.left.take(), value);
    } else if value > node.value {
        node.right = avl_insert(node.right.take(), value);
    } else {
        return Some(node);
    }

    node.height = 1 + std::cmp::max(node_height(&node.left), node_height(&node.right));
    let balance = balance_factor(&node);

    if balance > 1 && value < node.left.as_ref().unwrap().value {
        return Some(rotate_right(node));
    }
    if balance < -1 && value > node.right.as_ref().unwrap().value {
        return Some(rotate_left(node));
    }
    if balance > 1 && value > node.left.as_ref().unwrap().value {
        node.left = Some(rotate_left(node.left.take().unwrap()));
        return Some(rotate_right(node));
    }
    if balance < -1 && value < node.right.as_ref().unwrap().value {
        node.right = Some(rotate_right(node.right.take().unwrap()));
        return Some(rotate_left(node));
    }

    Some(node)
}

fn inorder(node: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, out);
        out.push(n.value);
        inorder(&n.right, out);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut root: Option<Box<Node>> = None;

    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        root = avl_insert(root, v);
    }

    let mut out = Vec::new();
    inorder(&root, &mut out);
    let strs: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{} ", strs.join(" "));
}
