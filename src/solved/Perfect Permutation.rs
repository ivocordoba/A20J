// mod scanner;
// use scanner::Scanner;
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
    let n: usize = scan.token();
    let mut res = vec![];
    if n % 2 != 0 {
        write!(out, "-1").ok();
        return;
    }
    let mut i = 2;
    while i <= n {
        res.push(i);
        res.push(i - 1);
        i += 2;
    }
    for num in res {
        write!(out, "{} ", num).ok();
    }
}
