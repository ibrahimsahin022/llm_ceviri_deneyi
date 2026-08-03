use std::io::{self, Read};

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Parser {
            chars: s.chars().collect(),
            pos: 0,
            _marker: std::marker::PhantomData,
        }
    }

    fn peek(&self) -> char {
        if self.pos < self.chars.len() {
            self.chars[self.pos]
        } else {
            '\0'
        }
    }

    fn skip_spaces(&mut self) {
        while self.peek() == ' ' || self.peek() == '\t' {
            self.pos += 1;
        }
    }

    fn match_word(&mut self, word: &str) -> bool {
        let wchars: Vec<char> = word.chars().collect();
        if self.pos + wchars.len() <= self.chars.len()
            && self.chars[self.pos..self.pos + wchars.len()] == wchars[..]
        {
            self.pos += wchars.len();
            true
        } else {
            false
        }
    }

    fn parse_factor(&mut self) -> bool {
        self.skip_spaces();
        if self.peek() == '(' {
            self.pos += 1;
            let v = self.parse_or();
            self.skip_spaces();
            if self.peek() == ')' {
                self.pos += 1;
            }
            return v;
        }
        if self.match_word("NOT") {
            let v = self.parse_factor();
            return !v;
        }
        if self.peek() == '0' {
            self.pos += 1;
            return false;
        }
        if self.peek() == '1' {
            self.pos += 1;
            return true;
        }
        false
    }

    fn parse_and(&mut self) -> bool {
        let mut value = self.parse_factor();
        loop {
            self.skip_spaces();
            if self.match_word("AND") {
                let rhs = self.parse_factor();
                value = value && rhs;
            } else {
                break;
            }
        }
        value
    }

    fn parse_or(&mut self) -> bool {
        let mut value = self.parse_and();
        loop {
            self.skip_spaces();
            if self.match_word("OR") {
                let rhs = self.parse_and();
                value = value || rhs;
            } else {
                break;
            }
        }
        value
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parser = Parser::new(&input);
    let result = parser.parse_or();
    println!("{}", if result { 1 } else { 0 });
}
