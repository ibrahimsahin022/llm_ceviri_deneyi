use std::io::{self, Read};

const ALPHABET: usize = 26;

struct TrieNode {
    children: [Option<Box<TrieNode>>; ALPHABET],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if idx >= ALPHABET {
                continue;
            }
            if cur.children[idx].is_none() {
                cur.children[idx] = Some(Box::new(TrieNode::new()));
            }
            cur = cur.children[idx].as_mut().unwrap();
        }
        cur.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut cur = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            if idx >= ALPHABET {
                return false;
            }
            match &cur.children[idx] {
                Some(node) => cur = node,
                None => return false,
            }
        }
        cur.is_end
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut root = TrieNode::new();

    for _ in 0..n {
        let op = it.next().unwrap();
        let word = it.next().unwrap();
        if op == "I" {
            root.insert(word);
        } else {
            println!("{}", if root.search(word) { "YES" } else { "NO" });
        }
    }
}
