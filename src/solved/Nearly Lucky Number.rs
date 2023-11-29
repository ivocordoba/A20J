use std::io::{self, prelude::*};
use std::str;
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let mut number: usize = scan.token();
    let mut lucky_numbers = 0;
    while number != 0 {
        let digit = number % 10;
        if digit == 4 || digit == 7 {
            lucky_numbers += 1;
        }
        number = (number - digit) / 10;
    }
    if lucky_numbers == 0 {
        write!(out, "NO").ok();
        return;
    }
    while lucky_numbers != 0 {
        let digit = lucky_numbers % 10;
        if digit != 4 && digit != 7 {
            write!(out, "NO").ok();
            return;
        }
        lucky_numbers = (lucky_numbers - digit) / 10;
    }
    write!(out, "YES").ok();
}
