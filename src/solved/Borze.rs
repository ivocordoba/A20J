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
    let b_code: String = scan.token();
    let b_code: Vec<char> = b_code.chars().collect();
    let mut decoded_b_code = String::from("");
    let mut i = 0;
    while i < b_code.len() {
        if b_code[i] == '.' {
            decoded_b_code.push('0');
            i += 1;
            continue;
        }
        if b_code[i] == '-' {
            if b_code[i + 1] == '.' {
                decoded_b_code.push('1')
            } else {
                // b_code[i+1] == "-"
                decoded_b_code.push('2')
            }
            i += 2
        }
    }
    write!(out, "{}", decoded_b_code).ok();
}
