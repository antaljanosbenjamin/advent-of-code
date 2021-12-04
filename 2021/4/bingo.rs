use std::fs;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;
const BOARD_SIZE: usize = WIDTH * HEIGHT;

type Board<T> = [T; BOARD_SIZE];

struct BoardRowIterator<'a, T> {
    board: &'a Board<T>,
    row: usize,
    current_index: usize,
}

impl<'a, T> BoardRowIterator<'a, T> {
    fn new(board: &'a Board<T>, row: usize) -> Option<Self> {
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

impl<'a, T> Iterator for BoardRowIterator<'a, T> {
    type Item = &'a T;

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

struct BoardColumnIterator<'a, T> {
    board: &'a Board<T>,
    column: usize,
    current_index: usize,
}

impl<'a, T> BoardColumnIterator<'a, T> {
    fn new(board: &'a Board<T>, column: usize) -> Option<Self> {
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

impl<'a, T> Iterator for BoardColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_index {
            x if x < HEIGHT => {
                let ret = Some(&self.board[self.column + HEIGHT * self.current_index]);
                self.current_index += 1;
                ret
            }
            _ => None,
        }
    }
}

type IterGetter<'a, T, I> = fn(&'a Board<T>, usize) -> Option<I>;

struct BoardIterator<'a, T, I> {
    board: &'a Board<T>,
    iterator_getter: IterGetter<'a, T, I>,
    current_index: usize,
}

impl<'a, T, I> BoardIterator<'a, T, I> {
    fn new(board: &'a Board<T>, iterator_getter: IterGetter<'a, T, I>) -> BoardIterator<'a, T, I> {
        Self {
            board,
            iterator_getter,
            current_index: 0,
        }
    }
}

impl<'a, T, I> Iterator for BoardIterator<'a, T, I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = (self.iterator_getter)(self.board, self.current_index);
        self.current_index += 1;
        ret
    }
}

trait IterableBoard<'a, T: 'a> {
    type RowIterator: Iterator<Item = &'a T>;
    type ColumnIterator: Iterator<Item = &'a T>;
    type RowsIterator: Iterator<Item = Self::RowIterator>;
    type ColumnsIterator: Iterator<Item = Self::ColumnIterator>;

    fn iter_row(&'a self, row_index: usize) -> Option<Self::RowIterator>;
    fn iter_col(&'a self, row_index: usize) -> Option<Self::ColumnIterator>;
    fn iter_rows(&'a self) -> Self::RowsIterator;
    fn iter_cols(&'a self) -> Self::ColumnsIterator;
}

impl<'a, T: 'a> IterableBoard<'a, T> for Board<T> {
    type RowIterator = BoardRowIterator<'a, T>;
    type ColumnIterator = BoardColumnIterator<'a, T>;
    type RowsIterator = BoardIterator<'a, T, Self::RowIterator>;
    type ColumnsIterator = BoardIterator<'a, T, Self::ColumnIterator>;

    fn iter_row(self: &'a Self, row_index: usize) -> Option<Self::RowIterator> {
        BoardRowIterator::new(self, row_index)
    }
    fn iter_col(self: &'a Self, column: usize) -> Option<Self::ColumnIterator> {
        BoardColumnIterator::new(self, column)
    }
    fn iter_rows(&'a self) -> Self::RowsIterator {
        BoardIterator::new(self, Self::iter_row)
    }
    fn iter_cols(&'a self) -> Self::ColumnsIterator {
        BoardIterator::new(self, Self::iter_col)
    }
}

type NumberBoard = Board<usize>;
type MarkBoard = Board<bool>;

fn part1(guesses: &Vec<usize>, num_boards: &Vec<NumberBoard>) -> usize {
    let mut mark_boards = vec![MarkBoard::default(); num_boards.len()];

    0
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

    let mut boards = vec![NumberBoard::default(); board_count];
    board_numbers
        .chunks(BOARD_SIZE)
        .zip(boards.iter_mut())
        .for_each(|(chunk, board)| board.copy_from_slice(chunk));

    println!("{:?}", guesses);
    println!("{:?}", boards[0]);
    boards[0]
        .iter_cols()
        .for_each(|row_it| println!("{:?}", row_it.collect::<Vec<&usize>>()));
}
