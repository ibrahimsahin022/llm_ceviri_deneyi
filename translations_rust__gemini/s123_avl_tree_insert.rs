use std::io::{self, Read};

struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn node_height(n: &Option<Box<Node>>) -> i32 {
    n.as_ref().map_or(0, |node| node.height)
}

fn update_height(node: &mut Node) {
    let lh = node_height(&node.left);
    let rh = node_height(&node.right);
    node.height = 1 + lh.max(rh);
}

fn balance_factor(node: &Node) -> i32 {
    node_height(&node.left) - node_height(&node.right)
}

fn rotate_right(mut y: Box<Node>) -> Box<Node> {
    let mut x = y.left.take().unwrap();
    let t2 = x.right.take();
    y.left = t2;
    update_height(&mut y);
    x.right = Some(y);
    update_height(&mut x);
    x
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.right.take().unwrap();
    let t2 = y.left.take();
    x.right = t2;
    update_height(&mut x);
    y.left = Some(x);
    update_height(&mut y);
    y
}

fn avl_insert(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    let mut node = match node {
        None => {
            return Some(Box::new(Node {
                value,
                height: 1,
                left: None,
                right: None,
            }));
        }
        Some(n) => n,
    };

    if value < node.value {
        node.left = avl_insert(node.left.take(), value);
    } else if value > node.value {
        node.right = avl_insert(node.right.take(), value);
    } else {
        return Some(node);
    }

    update_height(&mut node);
    let balance = balance_factor(&node);

    if balance > 1 {
        let left_val = node.left.as_ref().unwrap().value;
        if value < left_val {
            return Some(rotate_right(node));
        } else {
            node.left = Some(rotate_left(node.left.take().unwrap()));
            return Some(rotate_right(node));
        }
    }

    if balance < -1 {
        let right_val = node.right.as_ref().unwrap().value;
        if value > right_val {
            return Some(rotate_left(node));
        } else {
            node.right = Some(rotate_right(node.right.take().unwrap()));
            return Some(rotate_left(node));
        }
    }

    Some(node)
}

fn inorder(node: &Option<Box<Node>>) {
    if let Some(n) = node {
        inorder(&n.left);
        print!("{} ", n.value);
        inorder(&n.right);
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

    let mut root = None;
    for _ in 0..n {
        if let Some(v) = tokens.next().and_then(|s| s.parse().ok()) {
            root = avl_insert(root, v);
        } else {
            return;
        }
    }

    inorder(&root);
    println!();
}
