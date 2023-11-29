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

// ES UN XOR.
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let a: String = scan.token();
    let b: String = scan.token();
    let a_bin: Vec<char> = a.chars().collect();
    let b_bin: Vec<char> = b.chars().collect();
    let mut res = vec![];
    for i in 0..a_bin.len() {
        if a_bin[i] == b_bin[i] {
            res.push('0')
        } else {
            res.push('1')
        }
    }
    let res: String = res.iter().collect();
    write!(out, "{}", res).ok();
}
