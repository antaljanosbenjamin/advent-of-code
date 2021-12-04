use std::fs;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const BOARD_SIZE: usize = WIDTH * HEIGHT;

type Board = [usize; BOARD_SIZE];

struct BoardRowIterator<'a> {
    board: &'a Board,
    row: usize,
    current_index: usize,
}

struct BoardColumnIterator<'a> {
    board: &'a Board,
    column: usize,
    current_index: usize,
}

impl<'a> BoardRowIterator<'a> {
    fn new(board: &'a Board, row: usize) -> Option<Self> {
        if row < HEIGHT {
            Some(Self {
                board,
                row,
                current_index: 0,
            })
        } else {
            None
        }
    }
}

impl<'a> BoardColumnIterator<'a> {
    fn new(board: &'a Board, column: usize) -> Option<Self> {
        if column < WIDTH {
            Some(Self {
                board,
                column,
                current_index: 0,
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for BoardRowIterator<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index {
            x if x < WIDTH => {
                let ret = Some(&self.board[self.row * WIDTH + self.current_index]);
                self.current_index += 1;
                ret
            }
            _ => None,
        }
    }
}

impl<'a> Iterator for BoardColumnIterator<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index {
            x if x < HEIGHT => {
                let ret = Some(&self.board[self.column + WIDTH * self.current_index]);
                self.current_index += 1;
                ret
            }
            _ => None,
        }
    }
}

trait IterableBoard<'a, T: 'a> {
    type RowIterator: Iterator<Item = &'a T>;
    type ColumnIterator: Iterator<Item = &'a T>;

    fn row_iter(&'a self, row_index: usize) -> Option<Self::RowIterator>;
    fn col_iter(&'a self, row_index: usize) -> Option<Self::ColumnIterator>;
}

impl<'a> IterableBoard<'a, usize> for Board {
    type RowIterator = BoardRowIterator<'a>;
    type ColumnIterator = BoardColumnIterator<'a>;

    fn row_iter(self: &'a Self, row_index: usize) -> Option<BoardRowIterator<'a>> {
        BoardRowIterator::new(self, row_index)
    }
    fn col_iter(self: &'a Self, column: usize) -> Option<BoardColumnIterator<'a>> {
        BoardColumnIterator::new(self, column)
    }
}

fn main() {
    let file_content =
        fs::read_to_string("2021/4/input_small.txt").expect("Cannot read input file");
    let guesses = file_content
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|num_str| num_str.parse::<usize>().expect("Invalid number"))
        .collect::<Vec<usize>>();

    let board_numbers = file_content
        .split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse::<usize>().expect("invalid number"))
        .collect::<Vec<usize>>();

    let board_count = board_numbers.len() / BOARD_SIZE;
    assert_eq!(board_numbers.len(), board_count * BOARD_SIZE);
    board_numbers.iter();

    let mut boards = vec![Board::default(); board_count];
    board_numbers
        .chunks(BOARD_SIZE)
        .zip(boards.iter_mut())
        .for_each(|(chunk, board)| board.copy_from_slice(chunk));

    println!("{:?}", guesses);
    println!("{:?}", boards[0]);
}
