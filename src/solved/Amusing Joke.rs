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
    let fst: String = scan.token();
    let snd: String = scan.token();
    let thr: String = scan.token();
    let fst: Vec<char> = fst.chars().collect();
    let snd: Vec<char> = snd.chars().collect();
    let thr: Vec<char> = thr.chars().collect();
    let mut collection: std::collections::HashMap<char, isize> = std::collections::HashMap::new();
    for lettre in thr {
        match collection.get(&lettre) {
            Some(p) => collection.insert(lettre, p + 1),
            None => collection.insert(lettre, 1),
        };
    }
    for lettre in fst {
        match collection.get(&lettre) {
            Some(p) => {
                if *p < 0 {
                    write!(out, "NO").ok();
                    return;
                } else {
                    collection.insert(lettre, p - 1);
                }
            }
            None => {
                write!(out, "NO").ok();
                return;
            }
        }
    }
    for lettre in snd {
        match collection.get(&lettre) {
            Some(p) => {
                if *p < 0 {
                    write!(out, "NO").ok();
                    return;
                } else {
                    collection.insert(lettre, p - 1);
                }
            }
            None => {
                write!(out, "NO").ok();
                return;
            }
        }
    }
    for key in collection.keys() {
        match collection.get(key) {
            Some(p) => {
                if *p > 0 {
                    write!(out, "NO").ok();
                    return;
                }
            }
            // Nunca tendria que entrar en none porque estoty recorriendo las keys de el hashmap
            None => {
                write!(out, "NO").ok();
                return;
            }
        }
    }
    write!(out, "YES").ok();
}
