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
    let input: Vec<usize> = (0..n).map(|_| scan.token()).collect();
    let mut max = (input[0], 0);
    let mut min = (input[0], 0);
    for i in 1..input.len() {
        if input[i] > max.0 {
            max.0 = input[i];
            max.1 = i;
        }
        if input[i] <= min.0 {
            min.0 = input[i];
            min.1 = i;
        }
    }
    let min_steps = input.len() - 1 - min.1;
    let max_steps = max.1;
    if max.1 < min.1 {
        write!(out, "{}", max_steps + min_steps).ok();
    } else {
        write!(out, "{}", max_steps + min_steps - 1).ok();
    }
}
