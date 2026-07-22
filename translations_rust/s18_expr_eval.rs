use std::io::{self, Read};

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn skip(&mut self) {
        while self.pos < self.chars.len()
            && (self.chars[self.pos] == ' ' || self.chars[self.pos] == '\t')
        {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    // factor := number | '(' expr ')' | '-' factor | '+' factor
    fn factor(&mut self) -> i64 {
        self.skip();
        match self.peek() {
            Some('(') => {
                self.pos += 1;
                let v = self.expr();
                self.skip();
                if self.peek() == Some(')') {
                    self.pos += 1;
                }
                v
            }
            Some('-') => {
                self.pos += 1;
                -self.factor()
            }
            Some('+') => {
                self.pos += 1;
                self.factor()
            }
            _ => {
                let mut num: i64 = 0;
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        num = num * 10 + (c as i64 - '0' as i64);
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                num
            }
        }
    }

    // term := factor (('*' | '/') factor)*
    fn term(&mut self) -> i64 {
        let mut value = self.factor();
        loop {
            self.skip();
            match self.peek() {
                Some('*') => {
                    self.pos += 1;
                    value = value * self.factor();
                }
                Some('/') => {
                    self.pos += 1;
                    let d = self.factor();
                    value = value / d;
                }
                _ => break,
            }
        }
        value
    }

    // expr := term (('+' | '-') term)*
    fn expr(&mut self) -> i64 {
        let mut value = self.term();
        loop {
            self.skip();
            match self.peek() {
                Some('+') => {
                    self.pos += 1;
                    value = value + self.term();
                }
                Some('-') => {
                    self.pos += 1;
                    value = value - self.term();
                }
                _ => break,
            }
        }
        value
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut parser = Parser {
        chars: line.chars().collect(),
        pos: 0,
    };
    println!("{}", parser.expr());
}
