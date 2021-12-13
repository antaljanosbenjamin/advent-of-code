use std::{
    collections::HashSet,
    fs,
    io::{BufWriter, Write},
};

use common::print_solution;

fn distance(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

// +--> x
// |
// |
// V
// y

// [x, y]
type Coords = [usize; 2];

trait CoordsImpl {
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn horizontal(&self, other: &Self) -> bool;
    fn vertical(&self, other: &Self) -> bool;

    fn parallel_to_axles(&self, other: &Self) -> bool {
        return self.horizontal(other) || self.vertical(other);
    }

    fn fold(&self, line: &Self) -> Self;
}

impl CoordsImpl for Coords {
    fn x(&self) -> usize {
        self[0]
    }

    fn y(&self) -> usize {
        self[1]
    }

    fn horizontal(&self, other: &Self) -> bool {
        self.y() == other.y()
    }

    fn vertical(&self, other: &Self) -> bool {
        self.x() == other.x()
    }

    fn fold(&self, line: &Self) -> Self {
        assert!(line.x() == 0 || line.y() == 0);

        let fold_coord = |coord, fold| {
            if fold == 0 {
                coord
            } else {
                fold - distance(coord, fold)
            }
        };
        [
            fold_coord(self.x(), line.x()),
            fold_coord(self.y(), line.y()),
        ]
    }
}

fn print_activation_code(height: usize, width: usize, basins: &HashSet<Coords>) -> String {
    let mut solution = String::new();
    solution.push('\n');
    for row in 0..height {
        for col in 0..width {
            let char = if basins.contains(&[row, col]) {
                'X'
            } else {
                ' '
            };
            solution.push(char);
        }
        if row < height - 1 {
            solution.push('\n');
        }
    }
    solution
}

fn part1(dots: &HashSet<Coords>, folds: &Vec<Coords>) -> usize {
    let fold_to_use = folds[0];
    dots.iter()
        .map(|c| c.fold(&fold_to_use))
        .collect::<HashSet<Coords>>()
        .len()
}

fn part2(dots: &HashSet<Coords>, folds: &Vec<Coords>) -> String {
    let activation_code = folds
        .iter()
        .fold(dots.clone(), |dots, f| {
            dots.iter().map(|c| c.fold(f)).collect::<HashSet<Coords>>()
        })
        .iter()
        .map(|c| [c[1], c[0]])
        .collect::<HashSet<Coords>>();
    let width = activation_code.iter().map(|c| c[1]).max().unwrap();
    let height = activation_code.iter().map(|c| c[0]).max().unwrap();
    print_activation_code(height + 1, width + 1, &activation_code)
}

fn main() {
    let file_content = fs::read_to_string("2021/13/input.txt").expect("Cannot read input file");

    let dots: HashSet<Coords> = file_content
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .map(|cell| {
            let values = cell
                .split(',')
                .map(|num| num.parse::<usize>().expect("Invalid number"))
                .collect::<Vec<usize>>();
            [values[0], values[1]]
        })
        .collect();

    let folds = file_content
        .lines()
        .skip_while(|l| !l.starts_with("fold along "))
        .map(|l| l.split(' ').collect::<Vec<&str>>()[2])
        .map(|fold| {
            let values = fold.split('=').collect::<Vec<&str>>();
            let coord = values[1].parse::<usize>().expect("Unexpected input");
            if values[0] == "x" {
                [coord, 0]
            } else {
                [0, coord]
            }
        })
        .collect::<Vec<Coords>>();

    print_solution(1, part1(&dots, &folds));
    print_solution(2, part2(&dots, &folds));
}
