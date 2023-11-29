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
    let input: Vec<String> = (0..n).map(|_| scan.token()).collect();
    let mut res = vec![];
    for word in input {
        if word.len() > 10 {
            let splitted_word: Vec<char> = word.chars().collect();
            let abbr_word = format!(
                "{}{}{}",
                splitted_word[0],
                word.len() - 2,
                splitted_word[word.len() - 1]
            );
            res.push(abbr_word)
        } else {
            res.push(word)
        }
    }
    for abbr_word in res {
        write!(out, "{}\n", abbr_word).ok();
    }
}
