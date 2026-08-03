use std::io;

struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }
}

fn node_height(n: &Option<Box<Node>>) -> i32 {
    n.as_ref().map_or(0, |node| node.height)
}

fn max_int(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn rotate_right(mut y: Box<Node>) -> Box<Node> {
    let mut x = y.left.take().unwrap();
    y.left = x.right.take();
    x.right = Some(y);

    if let Some(ref mut right) = x.right {
        right.height = max_int(node_height(&right.left), node_height(&right.right)) + 1;
    }
    x.height = max_int(node_height(&x.left), node_height(&x.right)) + 1;

    x
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.right.take().unwrap();
    x.right = y.left.take();
    y.left = Some(x);

    if let Some(ref mut left) = y.left {
        left.height = max_int(node_height(&left.left), node_height(&left.right)) + 1;
    }
    y.height = max_int(node_height(&y.left), node_height(&y.right)) + 1;

    y
}

fn avl_insert(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    match node {
        None => Some(Box::new(Node::new(value))),
        Some(mut n) => {
            if value < n.value {
                n.left = avl_insert(n.left, value);
            } else if value > n.value {
                n.right = avl_insert(n.right, value);
            } else {
                return Some(n);
            }

            n.height = 1 + max_int(node_height(&n.left), node_height(&n.right));

            let balance = node_height(&n.left) - node_height(&n.right);

            // LL case
            if balance > 1 && value < n.left.as_ref().map_or(0, |l| l.value) {
                return Some(rotate_right(n));
            }

            // RR case
            if balance < -1 && value > n.right.as_ref().map_or(0, |r| r.value) {
                return Some(rotate_left(n));
            }

            // LR case
            if balance > 1 && value > n.left.as_ref().map_or(0, |l| l.value) {
                n.left = Some(rotate_left(n.left.take().unwrap()));
                return Some(rotate_right(n));
            }

            // RL case
            if balance < -1 && value < n.right.as_ref().map_or(0, |r| r.value) {
                n.right = Some(rotate_right(n.right.take().unwrap()));
                return Some(rotate_left(n));
            }

            Some(n)
        }
    }
}

fn inorder(n: &Option<Box<Node>>) {
    if let Some(node) = n {
        inorder(&node.left);
        print!("{} ", node.value);
        inorder(&node.right);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let n: usize = input.trim().parse().unwrap_or(0);

    let mut root: Option<Box<Node>> = None;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let v: i32 = input.trim().parse().unwrap_or(0);
        root = avl_insert(root, v);
    }

    inorder(&root);
    println!();
}
