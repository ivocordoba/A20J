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
    let splitted_input: Vec<char> = input.chars().collect();
    let mut hash: std::collections::HashMap<char, bool> = std::collections::HashMap::new();
    let mut distincts = 0;
    for lettre in splitted_input {
        match hash.get(&lettre) {
            Some(_) => {}
            None => {
                distincts += 1;
                hash.insert(lettre, true);
            }
        }
    }
    write!(
        out,
        "{}",
        if distincts % 2 == 0 {
            "CHAT WITH HER!"
        } else {
            "IGNORE HIM!"
        }
    )
    .ok();
}
