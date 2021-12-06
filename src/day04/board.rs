use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BoardError {
    #[error("Duplicate value in board initialization")]
    DuplicateNumber,
    #[error("Board must be 5x5")]
    InvalidBoardSize,
    #[error("Could not parse this string as board")]
    BoardParseError,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Board {
    number_to_coord: HashMap<u8, (usize, usize)>,
    marked: HashSet<(u8, u8)>
}

impl Board {
    fn from_vec_of_vecs(values: &[Vec<u8>]) -> Result<Board, BoardError> {
        let mut number_to_coord: HashMap<u8, (usize, usize)> = HashMap::new();
        if values.len() != 5 {
            return Err(BoardError::InvalidBoardSize)
        }
        if values.iter().any(|el| el.len() != 5) {
            return Err(BoardError::InvalidBoardSize)
        }
        
        for (i, row) in values.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if let Some(_) = number_to_coord.insert(*val, (i, j)) {
                    return Err(BoardError::DuplicateNumber)
                }
            }
        }
        
        let marked = HashSet::new();
        Ok(Board{number_to_coord, marked})
    }
}

fn parse_line(line: &str) -> Result<Vec<u8>, BoardError> {
    line.split_whitespace().map(|num| num.parse().map_err(|_e| BoardError::BoardParseError)).collect()
}

impl FromStr for Board {
    type Err = BoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_vec: Result<Vec<Vec<u8>>,_> = s.lines().map(|line| parse_line(line)).collect();
        Board::from_vec_of_vecs(&vec_vec?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::day04::board::BoardError;

    use super::Board;

    fn example_board() -> Board {
        let mut number_to_coord = HashMap::new();
        for i in 0..5 {
            for j in 0..5 {
                number_to_coord.insert((i*5 + j) as u8, (i, j));
            }
        }
        Board { number_to_coord: number_to_coord, marked: HashSet::new() }
    }

    #[test]
    fn it_creates_from_vec_vec() {
        let mut board_vec = Vec::new();
        for i in 0..5 {
            let row: Vec<u8> = (i*5..(i+1)*5).collect();
            board_vec.push(row);
        }
        let board = Board::from_vec_of_vecs(&board_vec).unwrap();
        assert_eq!(example_board(), board);
    }

    #[test]
    fn it_complains_about_dupliactes() {
        let mut board_vec = Vec::new();
        for _i in 0..5 {
            board_vec.push(vec![1,2,3,4,5]);
        }
        let res = Board::from_vec_of_vecs(&board_vec);
        assert_eq!(res, Err(BoardError::DuplicateNumber));
    }

    #[test]
    fn it_reads_board_from_string() {
        let str = "0 1 2 3 4\n5 6 7 8 9\n10 11 12 13 14\n15 16 17 18 19\n20 21 22 23 24";
        let board: Board = str.parse().unwrap();
        assert_eq!(example_board(), board);
    }
}