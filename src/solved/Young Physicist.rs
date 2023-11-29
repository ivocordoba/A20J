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

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
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
    let mut partial_sum = vec![0, 0, 0];
    for _ in 0..n {
        let vector: Vec<isize> = (0..3).map(|_| scan.token()).collect();
        partial_sum[0] = partial_sum[0] + vector[0];
        partial_sum[1] = partial_sum[1] + vector[1];
        partial_sum[2] = partial_sum[2] + vector[2];
    }
    if partial_sum[0] != 0 || partial_sum[1] != 0 || partial_sum[2] != 0 {
        write!(out, "NO").ok();
    } else {
        write!(out, "YES").ok();
    }
}
