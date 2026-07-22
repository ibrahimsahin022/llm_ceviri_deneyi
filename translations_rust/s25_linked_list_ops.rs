use std::io::{self, Read};

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn append(head: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match head {
        None => Some(Box::new(Node { value: v, next: None })),
        Some(mut n) => {
            n.next = append(n.next, v);
            Some(n)
        }
    }
}

fn reverse_list(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut cur) = head {
        head = cur.next.take();
        cur.next = prev;
        prev = Some(cur);
    }
    prev
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|t| t.parse::<i32>().unwrap());

    let n = it.next().unwrap();
    let mut head: Option<Box<Node>> = None;
    for _ in 0..n {
        let v = it.next().unwrap();
        head = append(head, v);
    }

    head = reverse_list(head);

    let mut sum: i64 = 0;
    let mut have_min = false;
    let mut mn = 0;
    let mut mx = 0;
    let mut parts: Vec<String> = Vec::new();

    let mut cur = &head;
    while let Some(node) = cur {
        parts.push(node.value.to_string());
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

    println!("{}", parts.join(" "));
    if have_min {
        println!("sum={} min={} max={}", sum, mn, mx);
    } else {
        println!("sum=0");
    }
}
