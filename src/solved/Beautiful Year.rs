use std::collections::HashMap;
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
    let y: usize = scan.token();
    let mut i = y + 1;
    loop {
        let mut digits_amount: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut j = i;
        let mut flag = false;
        while j != 0 {
            let last_digit = j % 10;
            if digits_amount[last_digit] == 1 {
                flag = true;
                break;
            } else {
                digits_amount[last_digit] = 1;
            }
            j = (j - last_digit) / 10;
        }
        if flag {
            i += 1;
            continue;
        } else {
            write!(out, "{}", i).ok();
            return;
        }
    }
}
