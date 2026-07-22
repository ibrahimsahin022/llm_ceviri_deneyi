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
        while self.peek() == b' ' || self.peek() == b'\t' {
            self.pos += 1;
        }
    }

    fn parse_factor(&mut self) -> i64 {
        self.skip_spaces();
        let ch = self.peek();
        if ch == b'(' {
            self.pos += 1;
            let v = self.parse_expr();
            self.skip_spaces();
            if self.peek() == b')' {
                self.pos += 1;
            }
            return v;
        }
        if ch == b'-' {
            self.pos += 1;
            return self.parse_factor().wrapping_neg();
        }
        if ch == b'+' {
            self.pos += 1;
            return self.parse_factor();
        }
        let mut num: i64 = 0;
        while self.peek().is_ascii_digit() {
            num = num.wrapping_mul(10).wrapping_add((self.peek() - b'0') as i64);
            self.pos += 1;
        }
        num
    }

    fn parse_term(&mut self) -> i64 {
        let mut value = self.parse_factor();
        loop {
            self.skip_spaces();
            let ch = self.peek();
            if ch == b'*' {
                self.pos += 1;
                value = value.wrapping_mul(self.parse_factor());
            } else if ch == b'/' {
                self.pos += 1;
                let d = self.parse_factor();
                value = value.wrapping_div(d);
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
            let ch = self.peek();
            if ch == b'+' {
                self.pos += 1;
                value = value.wrapping_add(self.parse_term());
            } else if ch == b'-' {
                self.pos += 1;
                value = value.wrapping_sub(self.parse_term());
            } else {
                break;
            }
        }
        value
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    if stdin.lock().read_line(&mut line).unwrap_or(0) > 0 {
        let mut parser = Parser::new(line.as_bytes());
        let result = parser.parse_expr();
        println!("{}", result);
    }
}
