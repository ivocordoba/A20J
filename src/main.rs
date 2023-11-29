use std::io::{self, prelude::*};
use std::str;
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let n_array: usize = scan.token();
    let input_vec: Vec<usize> = (0..n_array).map(|_| scan.token()).collect();
    let n_query: usize = scan.token();
    let input_query: Vec<usize> = (0..n_query).map(|_| scan.token()).collect();
    // O(N/2*M)
    // ES O()
    let mut v_com = 0;
    let mut p_com = 0;
    for query in input_query {
        let mut i = 0;
        let mut j = input_vec.len() - 1;
        let mut j_end = false;
        let mut i_end = false;
        loop {
            if input_vec[i] == query {
                i_end = true;
                break;
            }
            i += 1;
            if input_vec[j] == query {
                j_end = true;
                break;
            }
            j -= 1;
        }
        if i_end {
            v_com += i + 1;
            p_com += input_vec.len() - i;
            continue;
        }
        if j_end {
            p_com += input_vec.len() - j;
            v_com += j + 1;
            continue;
        }
    }
    write!(out, "{} {}", v_com, p_com).ok();
}
