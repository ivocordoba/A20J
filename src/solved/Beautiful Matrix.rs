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
    let mut one_pos: (usize, usize) = (0, 0);
    for i in 0..5 {
        let _: Vec<usize> = (0..5)
            .map(|j| {
                let n: usize = scan.token();
                if n == 1 {
                    one_pos.0 = i;
                    one_pos.1 = j;
                }
                n
            })
            .collect();
    }
    let mut moves = 0;
    while one_pos.0 != 2 {
        if one_pos.0 > 2 {
            one_pos.0 = one_pos.0 - 1
        } else {
            // one_pos.0 no puede ser 2, osea one_pos.0 < 2
            one_pos.0 = one_pos.0 + 1;
        }
        moves += 1;
    }
    while one_pos.1 != 2 {
        if one_pos.1 > 2 {
            one_pos.1 = one_pos.1 - 1
        } else {
            // one_pos.1 no puede ser 2, osea one_pos.0 < 2
            one_pos.1 = one_pos.1 + 1;
        }
        moves += 1;
    }
    write!(out, "{}", moves).ok();
}
