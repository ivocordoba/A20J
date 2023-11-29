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

fn get_pos((i, j): (usize, usize), m_dim: isize) -> Vec<(usize, usize)> {
    let i_integer = i as isize;
    let j_integer = j as isize;
    let mut res = vec![];
    // fst i fixed
    if j_integer - 1 >= 0 {
        res.push((i, j - 1))
    }
    if j_integer + 1 < m_dim {
        res.push((i, j + 1))
    }
    // j fixed
    if i_integer - 1 >= 0 {
        res.push((i - 1, j))
    }
    if i_integer + 1 < m_dim {
        res.push((i + 1, j))
    }
    // diag ACA ESTA MEDIO MAL EXPLICADO, ENCIMA NO CONSIDERA LA DIAGONAL COMO POSICION ADYACENTE.
    // if i_integer - 1 >= 0 && j_integer - 1 >= 0 {
    //     res.push((i - 1, j - 1));
    // }
    // if i_integer - 1 >= 0 && j_integer + 1 < m_dim {
    //     res.push((i - 1, j + 1))
    // }
    // if i_integer + 1 < m_dim && j_integer - 1 >= 0 {
    //     res.push((i + 1, j - 1))
    // }
    // if i_integer + 1 < m_dim && j_integer + 1 < m_dim {
    //     res.push((i + 1, j + 1));
    // }
    // i, j always change
    res.push((i, j));
    res
}

// fn main() {
//     let (stdin, stdout) = (io::stdin(), io::stdout());
//     let mut scan = Scanner::new(stdin.lock());
//     let mut out = io::BufWriter::new(stdout.lock());
//     let mut init_game: Vec<Vec<usize>> = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
//     let mut played_game: Vec<Vec<usize>> = vec![];
//     for _ in 0..3 {
//         let row: Vec<usize> = (0..3).map(|_| scan.token()).collect();
//         played_game.push(row);
//     }
//     for (i, _) in played_game.iter().enumerate() {
//         for (j, _) in played_game[i].iter().enumerate() {
//             println!("(i, j) = ({}, {})", i, j);
//             let mut quant = played_game[i][j];
//             let to_change = get_pos((i, j), 3);
//             println!("to_change = {:?}", to_change);
//             while quant > 0 {
//                 for pos in &to_change {
//                     init_game[pos.0][pos.1] = if init_game[pos.0][pos.1] == 0 { 1 } else { 0 }
//                 }
//                 quant -= 1;
//                 // println!("init_game = {:?}", init_game);
//             }
//         }
//     }
//     println!("final_game = {:?}", init_game);
// }

//TARDE 70 HORAS PARA DARME CUENTA QUE SI LA POSICION ES PAR QUEDA TODO IGUAL Y SI ES IMPAR CAMBIA....
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    let mut init_game: Vec<Vec<usize>> = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
    let mut played_game: Vec<Vec<usize>> = vec![];
    for _ in 0..3 {
        let row: Vec<usize> = (0..3).map(|_| scan.token()).collect();
        played_game.push(row);
    }
    for (i, _) in played_game.iter().enumerate() {
        for (j, _) in played_game[i].iter().enumerate() {
            let quant = played_game[i][j];
            // Si el numero es par el juego no cambia, se vuelve a la misma posicione,
            // si es impar cambia, pero solo 1 vez.
            if quant % 2 != 0 {
                let to_change = get_pos((i, j), 3);
                for (i, j) in to_change {
                    init_game[i][j] = if init_game[i][j] == 0 { 1 } else { 0 }
                }
            }
        }
    }
    write!(
        out,
        "{}{}{}\n{}{}{}\n{}{}{}",
        init_game[0][0],
        init_game[0][1],
        init_game[0][2],
        init_game[1][0],
        init_game[1][1],
        init_game[1][2],
        init_game[2][0],
        init_game[2][1],
        init_game[2][2]
    )
    .ok();
}
