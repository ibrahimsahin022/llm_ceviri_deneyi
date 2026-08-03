use std::io::{self, Read};

const ALPHABET: usize = 26;

struct TrieNode {
    children: [Option<Box<TrieNode>>; ALPHABET],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self {
            children: [NONE; ALPHABET],
            is_end: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for &byte in word.as_bytes() {
            let idx = (byte as i32) - (b'a' as i32);
            if idx < 0 || idx >= ALPHABET as i32 {
                continue;
            }
            let idx = idx as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        cur.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut cur = self;
        for &byte in word.as_bytes() {
            let idx = (byte as i32) - (b'a' as i32);
            if idx < 0 || idx >= ALPHABET as i32 {
                return false;
            }
            let idx = idx as usize;
            match &cur.children[idx] {
                Some(child) => cur = child,
                None => return false,
            }
        }
        cur.is_end
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: usize = match tokens.next() {
        Some(s) => match s.parse() {
            Ok(val) => val,
            Err(_) => return,
        },
        None => return,
    };

    let mut root = TrieNode::new();

    for _ in 0..n {
        let op = match tokens.next() {
            Some(s) => s,
            None => return,
        };
        let word = match tokens.next() {
            Some(s) => s,
            None => return,
        };

        if op.starts_with('I') {
            root.insert(word);
        } else {
            if root.search(word) {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
