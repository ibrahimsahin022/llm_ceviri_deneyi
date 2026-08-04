use std::io::{self, BufRead};

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a [u8]) -> Self {
        Parser { s, pos: 0 }
    }

    fn peek(&self) -> u8 {
        if self.pos < self.s.len() {
            self.s[self.pos]
        } else {
            0
        }
    }

    fn skip_spaces(&mut self) {
        while self.pos < self.s.len() && (self.s[self.pos] == b' ' || self.s[self.pos] == b'\t') {
            self.pos += 1;
        }
    }

    fn match_word(&mut self, word: &[u8]) -> bool {
        let len = word.len();
        if self.pos + len <= self.s.len() && &self.s[self.pos..self.pos + len] == word {
            self.pos += len;
            true
        } else {
            false
        }
    }

    fn parse_factor(&mut self) -> i32 {
        self.skip_spaces();
        if self.peek() == b'(' {
            self.pos += 1;
            let v = self.parse_or();
            self.skip_spaces();
            if self.peek() == b')' {
                self.pos += 1;
            }
            return v;
        }
        if self.match_word(b"NOT") {
            let v = self.parse_factor();
            return if v == 0 { 1 } else { 0 };
        }
        if self.peek() == b'0' {
            self.pos += 1;
            return 0;
        }
        if self.peek() == b'1' {
            self.pos += 1;
            return 1;
        }
        0
    }

    fn parse_and(&mut self) -> i32 {
        let mut value = self.parse_factor();
        loop {
            self.skip_spaces();
            if self.match_word(b"AND") {
                let rhs = self.parse_factor();
                value = if (value != 0) && (rhs != 0) { 1 } else { 0 };
            } else {
                break;
            }
        }
        value
    }

    fn parse_or(&mut self) -> i32 {
        let mut value = self.parse_and();
        loop {
            self.skip_spaces();
            if self.match_word(b"OR") {
                let rhs = self.parse_and();
                value = if (value != 0) || (rhs != 0) { 1 } else { 0 };
            } else {
                break;
            }
        }
        value
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = Vec::new();

    match handle.read_until(b'\n', &mut line) {
        Ok(0) | Err(_) => return,
        Ok(_) => {}
    }

    if line.len() >= 4096 {
        line.truncate(4095);
    }

    let len = line.iter().position(|&b| b == 0).unwrap_or(line.len());
    let bytes = &line[..len];

    let mut parser = Parser::new(bytes);
    let result = parser.parse_or();
    println!("{}", result);
}
