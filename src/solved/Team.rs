use std::io::{self, prelude::*};
use std::{str, vec};
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
    let mut input: Vec<Vec<usize>> = vec![];
    let mut res = 0;
    for _ in 0..n {
        let t: Vec<usize> = (0..3).map(|_| scan.token()).collect();
        input.push(t);
    }
    for contest in input {
        let mut sec = 0;
        for n in contest {
            if n == 1 {
                sec += 1
            }
        }
        if sec >= 2 {
            res += 1;
        }
    }
    write!(out, "{}", res).ok();
}
