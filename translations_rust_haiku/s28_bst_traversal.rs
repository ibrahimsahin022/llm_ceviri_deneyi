use std::io::{self, BufRead};
use std::cell::RefCell;
use std::rc::Rc;

type NodePtr = Option<Rc<RefCell<Node>>>;

struct Node {
    value: i32,
    left: NodePtr,
    right: NodePtr,
}

fn make_node(v: i32) -> NodePtr {
    Some(Rc::new(RefCell::new(Node {
        value: v,
        left: None,
        right: None,
    })))
}

fn insert(root: NodePtr, v: i32) -> NodePtr {
    match root {
        None => make_node(v),
        Some(node) => {
            let mut n = node.borrow_mut();
            if v < n.value {
                n.left = insert(n.left.take(), v);
            } else {
                n.right = insert(n.right.take(), v);
            }
            drop(n);
            Some(node)
        }
    }
}

fn count_nodes(root: &NodePtr) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let n = node.borrow();
            1 + count_nodes(&n.left) + count_nodes(&n.right)
        }
    }
}

fn height(root: &NodePtr) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let n = node.borrow();
            let lh = height(&n.left);
            let rh = height(&n.right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

fn inorder(root: &NodePtr, first: &mut bool) {
    match root {
        None => {}
        Some(node) => {
            let n = node.borrow();
            inorder(&n.left, first);
            if !*first {
                print!(" ");
            }
            print!("{}", n.value);
            *first = false;
            inorder(&n.right, first);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut root: NodePtr = None;
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(v) = line.trim().parse::<i32>() {
                root = insert(root, v);
            }
        }
    }

    let mut first = true;
    inorder(&root, &mut first);
    println!();

    println!("count={} height={}", count_nodes(&root), height(&root));
}
