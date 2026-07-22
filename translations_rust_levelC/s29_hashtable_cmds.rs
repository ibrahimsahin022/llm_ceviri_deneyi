use std::io::{self, Read};

const TABLE_SIZE: usize = 64;

struct Entry {
    key: String,
    value: i64,
    next: Option<Box<Entry>>,
}

fn hash_key(key: &str) -> u64 {
    let mut h: u64 = 5381;
    for c in key.bytes() {
        h = (h << 5) + h + c as u64;
    }
    h
}

struct HashTable {
    buckets: Vec<Option<Box<Entry>>>,
}

impl HashTable {
    fn new() -> Self {
        let mut buckets = Vec::with_capacity(TABLE_SIZE);
        for _ in 0..TABLE_SIZE {
            buckets.push(None);
        }
        HashTable { buckets }
    }

    fn idx(&self, key: &str) -> usize {
        (hash_key(key) % TABLE_SIZE as u64) as usize
    }

    fn get(&self, key: &str) -> Option<i64> {
        let mut cur = &self.buckets[self.idx(key)];
        while let Some(entry) = cur {
            if entry.key == key {
                return Some(entry.value);
            }
            cur = &entry.next;
        }
        None
    }

    fn insert(&mut self, key: &str, value: i64) {
        let i = self.idx(key);
        let mut cur = &mut self.buckets[i];
        while let Some(entry) = cur {
            if entry.key == key {
                entry.value = value;
                return;
            }
            cur = &mut entry.next;
        }
        let new_entry = Box::new(Entry {
            key: key.to_string(),
            value,
            next: self.buckets[i].take(),
        });
        self.buckets[i] = Some(new_entry);
    }

    fn delete(&mut self, key: &str) -> bool {
        let i = self.idx(key);
        let mut cur = &mut self.buckets[i];
        loop {
            match cur {
                None => return false,
                Some(entry) if entry.key == key => {
                    *cur = entry.next.take();
                    return true;
                }
                Some(entry) => {
                    cur = &mut entry.next;
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let k: usize = lines.next().unwrap_or("0").trim().parse().unwrap_or(0);
    let mut table = HashTable::new();

    for _ in 0..k {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        match tokens[0] {
            "INSERT" if tokens.len() >= 2 => {
                let key = tokens[1];
                let value: i64 = if tokens.len() >= 3 {
                    tokens[2].parse().unwrap_or(0)
                } else {
                    0
                };
                table.insert(key, value);
                println!("OK");
            }
            "GET" if tokens.len() >= 2 => {
                let key = tokens[1];
                match table.get(key) {
                    Some(v) => println!("{}", v),
                    None => println!("NOT_FOUND"),
                }
            }
            "DEL" if tokens.len() >= 2 => {
                let key = tokens[1];
                if table.delete(key) {
                    println!("OK");
                } else {
                    println!("NOT_FOUND");
                }
            }
            _ => {}
        }
    }
}
