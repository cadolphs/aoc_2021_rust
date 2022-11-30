use std::{error::Error, str::FromStr};

use crate::inputs::read_file_to_list;

pub fn run_day_04() {
    let filename = "data/day04.txt";
    println!("Well. Nothing here yet.");
}

pub struct Board {
    marked_numbers: Vec<i32>,
    board: [i32; 25]
}

impl Board {
    fn get_test_board() -> Self {
        let board = (1..26).collect::<Vec<i32>>().try_into().unwrap();
        return Board {marked_numbers: Vec::new(), board};
    }

    fn mark_number(&mut self, number: i32) {
        if self.board.contains(&number) {
            self.marked_numbers.push(number);
        }
    }

    fn get_marked_numbers(&self) -> impl Iterator<Item=i32> + '_ {
        self.marked_numbers.iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::day04::Board;

    #[test]
    fn it_sets_a_square_as_marked_if_present()
    {
        let mut board = Board::get_test_board();

        board.mark_number(5);

        let marked_numbers: Vec<i32> = board.get_marked_numbers().collect();

        assert_eq!(vec![5], marked_numbers);
    }

    #[test]
    fn it_doesnt_set_a_square_as_marked_if_not_present()
    {
        let mut board = Board::get_test_board();
        board.mark_number(42);

        let marked_numbers: Vec<i32> = board.get_marked_numbers().collect();

        assert_eq!(Vec::<i32>::new(), marked_numbers);
    }
}
