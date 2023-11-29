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
    let t: usize = scan.token();
    let children: String = scan.token();
    let mut children: Vec<char> = children.chars().collect();
    let mut i = 0;
    for _ in 0..t {
        // While O(n_input)
        while i < n - 1 {
            if children[i] == 'B' && children[i + 1] == 'G' {
                children[i] = 'G';
                children[i + 1] = 'B';
                i += 2;
            } else {
                i += 1;
            }
        }
        i = 0;
    }

    let children: String = children.iter().collect();
    write!(out, "{}", children).ok();
}
