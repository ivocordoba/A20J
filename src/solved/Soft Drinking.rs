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
    let n_friend: usize = scan.token();
    let k_bottle: usize = scan.token();
    let l_ml_bottle: usize = scan.token();
    let c_limes: usize = scan.token();
    let d_limes_slice: usize = scan.token();
    let p_gram_salt: usize = scan.token();
    let nl: usize = scan.token();
    let np: usize = scan.token();
    let total_ml = k_bottle * l_ml_bottle;
    let ml_bound = total_ml / nl;
    let salt_bound = p_gram_salt / np;
    let slice_bound = c_limes * d_limes_slice;
    let max_p_friend = std::cmp::min(std::cmp::min(ml_bound, salt_bound), slice_bound);
    write!(out, "{}", max_p_friend / n_friend).ok();
}
