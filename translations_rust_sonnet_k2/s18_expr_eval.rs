use std::io::{self, Read};

struct Parser {
    buf: Vec<u8>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> u8 {
        if self.pos < self.buf.len() {
            self.buf[self.pos]
        } else {
            0
        }
    }

    fn advance(&mut self) {
        if self.pos < self.buf.len() {
            self.pos += 1;
        }
    }

    fn skip_spaces(&mut self) {
        while self.peek() == b' ' || self.peek() == b'\t' {
            self.advance();
        }
    }

    fn parse_factor(&mut self) -> i64 {
        self.skip_spaces();
        if self.peek() == b'(' {
            self.advance();
            let v = self.parse_expr();
            self.skip_spaces();
            if self.peek() == b')' {
                self.advance();
            }
            return v;
        }
        if self.peek() == b'-' {
            self.advance();
            return -self.parse_factor();
        }
        if self.peek() == b'+' {
            self.advance();
            return self.parse_factor();
        }
        let mut num: i64 = 0;
        while self.peek().is_ascii_digit() {
            num = num * 10 + (self.peek() - b'0') as i64;
            self.advance();
        }
        num
    }

    fn parse_term(&mut self) -> i64 {
        let mut value = self.parse_factor();
        loop {
            self.skip_spaces();
            if self.peek() == b'*' {
                self.advance();
                value *= self.parse_factor();
            } else if self.peek() == b'/' {
                self.advance();
                let d = self.parse_factor();
                value /= d;
            } else {
                break;
            }
        }
        value
    }

    fn parse_expr(&mut self) -> i64 {
        let mut value = self.parse_term();
        loop {
            self.skip_spaces();
            if self.peek() == b'+' {
                self.advance();
                value += self.parse_term();
            } else if self.peek() == b'-' {
                self.advance();
                value -= self.parse_term();
            } else {
                break;
            }
        }
        value
    }
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    if input.is_empty() {
        return;
    }
    let mut end = 0usize;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    let line = input[..end].to_vec();

    let mut parser = Parser { buf: line, pos: 0 };
    let result = parser.parse_expr();

    println!("{}", result);
}
