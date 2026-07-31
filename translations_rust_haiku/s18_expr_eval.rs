use std::io::{self, BufRead};

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Parser {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    fn skip_spaces(&mut self) {
        while self.pos < self.input.len() && (self.input[self.pos] == ' ' || self.input[self.pos] == '\t') {
            self.pos += 1;
        }
    }

    fn current(&self) -> char {
        if self.pos < self.input.len() {
            self.input[self.pos]
        } else {
            '\0'
        }
    }

    fn parse_factor(&mut self) -> i64 {
        self.skip_spaces();
        if self.current() == '(' {
            self.pos += 1;
            let v = self.parse_expr();
            self.skip_spaces();
            if self.current() == ')' {
                self.pos += 1;
            }
            return v;
        }
        if self.current() == '-' {
            self.pos += 1;
            return -self.parse_factor();
        }
        if self.current() == '+' {
            self.pos += 1;
            return self.parse_factor();
        }
        let mut num: i64 = 0;
        while self.pos < self.input.len() && self.input[self.pos].is_numeric() {
            num = num * 10 + (self.input[self.pos] as i64 - '0' as i64);
            self.pos += 1;
        }
        num
    }

    fn parse_term(&mut self) -> i64 {
        let mut value = self.parse_factor();
        loop {
            self.skip_spaces();
            if self.current() == '*' {
                self.pos += 1;
                value = value * self.parse_factor();
            } else if self.current() == '/' {
                self.pos += 1;
                let d = self.parse_factor();
                value = value / d;
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
            if self.current() == '+' {
                self.pos += 1;
                value = value + self.parse_term();
            } else if self.current() == '-' {
                self.pos += 1;
                value = value - self.parse_term();
            } else {
                break;
            }
        }
        value
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut parser = Parser::new(&line);
        let result = parser.parse_expr();
        println!("{}", result);
    }
}
