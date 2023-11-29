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
    let input: String = scan.token();
    let mut splitted_input: Vec<usize> = input
        .split('+')
        .map(|s_n| s_n.parse::<usize>().unwrap())
        .collect();
    splitted_input.sort();
    let sorted_sum: Vec<String> = splitted_input.iter().map(|num| num.to_string()).collect();
    let res = sorted_sum.join("+");
    write!(out, "{}", res).ok();
}
