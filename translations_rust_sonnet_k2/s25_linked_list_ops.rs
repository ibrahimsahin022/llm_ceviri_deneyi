use std::io::{self, Read};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn append(head: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    let new_node = Box::new(Node { value: v, next: None });
    match head {
        None => Some(new_node),
        Some(mut h) => {
            let mut cur = &mut h;
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(new_node);
            Some(h)
        }
    }
}

fn reverse_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        cur = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
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

    let mut head: Option<Box<Node>> = None;
    for _ in 0..n {
        let v: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        head = append(head, v);
    }

    head = reverse_list(head);

    let mut sum: i64 = 0;
    let mut have_min = false;
    let mut mn: i32 = 0;
    let mut mx: i32 = 0;
    let mut first = true;

    let mut out = String::new();
    let mut cur = head.as_deref();
    while let Some(node) = cur {
        if !first {
            out.push(' ');
        }
        out.push_str(&node.value.to_string());
        first = false;
        sum += node.value as i64;
        if !have_min {
            mn = node.value;
            mx = node.value;
            have_min = true;
        } else {
            if node.value < mn {
                mn = node.value;
            }
            if node.value > mx {
                mx = node.value;
            }
        }
        cur = node.next.as_deref();
    }
    out.push('\n');
    print!("{}", out);

    if have_min {
        println!("sum={} min={} max={}", sum, mn, mx);
    } else {
        println!("sum=0");
    }
}
