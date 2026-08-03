use std::io;

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
}

fn trie_insert(node: &mut TrieNode, word: &[u8]) {
    if word.is_empty() {
        node.is_end = true;
        return;
    }
    let c = word[0];
    let idx = (c - b'a') as usize;
    if idx >= ALPHABET {
        return;
    }
    if node.children[idx].is_none() {
        node.children[idx] = Some(Box::new(TrieNode::new()));
    }
    trie_insert(node.children[idx].as_mut().unwrap(), &word[1..]);
}

fn trie_search(node: &TrieNode, word: &[u8]) -> bool {
    if word.is_empty() {
        return node.is_end;
    }
    let c = word[0];
    let idx = (c - b'a') as usize;
    if idx >= ALPHABET {
        return false;
    }
    match &node.children[idx] {
        Some(child) => trie_search(child, &word[1..]),
        None => false,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let n: usize = input.trim().parse().unwrap_or(0);

    let mut root = TrieNode::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let op = parts[0];
        let word = parts[1];

        if op == "I" {
            trie_insert(&mut root, word.as_bytes());
        } else {
            println!("{}", if trie_search(&root, word.as_bytes()) { "YES" } else { "NO" });
        }
    }
}
