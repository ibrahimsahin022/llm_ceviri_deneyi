use std::io::{self, BufRead};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn append(head: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match head {
        None => Some(Box::new(Node { value: v, next: None })),
        Some(mut node) => {
            let mut cur = &mut node;
            while cur.next.is_some() {
                cur = &mut cur.next.as_mut().unwrap();
            }
            cur.next = Some(Box::new(Node { value: v, next: None }));
            Some(node)
        }
    }
}

fn reverse_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    let mut cur = head;
    while let Some(mut node) = cur {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        cur = next;
    }
    prev
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut head: Option<Box<Node>> = None;
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(v) = line.trim().parse::<i32>() {
                head = append(head, v);
            }
        }
    }

    head = reverse_list(head);

    let mut sum: i64 = 0;
    let mut have_min = false;
    let mut mn: i32 = 0;
    let mut mx: i32 = 0;
    let mut first = true;

    let mut cur = &head;
    while let Some(node) = cur {
        if !first {
            print!(" ");
        }
        print!("{}", node.value);
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
        cur = &node.next;
    }
    println!();

    if have_min {
        println!("sum={} min={} max={}", sum, mn, mx);
    } else {
        println!("sum=0");
    }
}
