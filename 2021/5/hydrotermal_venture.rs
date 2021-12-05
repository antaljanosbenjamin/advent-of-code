use std::{
    cmp::Ordering,
    fs,
    iter::{self, repeat, Zip},
};

use common::print_solution;

const ARROW: &'static str = " -> ";

// +--> x
// |
// |
// V
// y

// [x, y]
type Coords = [usize; 2];
type Segment = [Coords; 2];

trait CoordsImpl {
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn horizontal(&self, other: &Self) -> bool;
    fn vertical(&self, other: &Self) -> bool;

    fn parallel_to_axles(&self, other: &Self) -> bool {
        return self.horizontal(other) || self.vertical(other);
    }
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
}

trait SegmentImpl {
    fn from(&self) -> &Coords;
    fn to(&self) -> &Coords;

    fn horizontal(&self) -> bool;
    fn vertical(&self) -> bool;

    fn parallel_to_axles(&self) -> bool {
        return self.horizontal() || self.vertical();
    }

    fn min_x(&self) -> usize {
        self.from().x().min(self.to().x())
    }

    fn min_y(&self) -> usize {
        self.from().y().min(self.to().y())
    }

    fn max_x(&self) -> usize {
        self.from().x().max(self.to().x())
    }

    fn max_y(&self) -> usize {
        self.from().y().max(self.to().y())
    }

    fn iter(&self) -> Zip<Box<dyn Iterator<Item = usize>>, Box<dyn Iterator<Item = usize>>> {
        let from = self.from();
        let to = self.to();
        let (start, end) = match from.x().partial_cmp(&to.x()).unwrap() {
            Ordering::Equal => {
                if from.y() < to.y() {
                    (from, to)
                } else {
                    (to, from)
                }
            }
            Ordering::Less => (from, to),
            Ordering::Greater => (to, from),
        };

        let get_range: fn(usize, usize) -> Box<dyn Iterator<Item = usize>> = |start, end| {
            if start == end {
                Box::new(repeat(start))
            } else if start > end {
                Box::new((end..=start).rev())
            } else {
                Box::new(start..=end)
            }
        };
        let x_range = get_range(start.x(), end.x());
        let y_range = get_range(start.y(), end.y());
        (x_range).zip(y_range)
    }
}

impl SegmentImpl for Segment {
    fn from(&self) -> &Coords {
        &self[0]
    }

    fn to(&self) -> &Coords {
        &self[1]
    }

    fn horizontal(&self) -> bool {
        self.from().horizontal(self.to())
    }

    fn vertical(&self) -> bool {
        self.from().vertical(self.to())
    }
}

#[allow(dead_code)]
fn println_segment(segment: &Segment) {
    println!(
        "{},{} -> {},{}",
        segment[0][0], segment[0][1], segment[1][0], segment[1][1]
    );
}

#[allow(dead_code)]
fn println_ocean_floor(ocean_floor: &Vec<Vec<usize>>) {
    let max_x = ocean_floor.len();
    let max_y = ocean_floor[0].len();
    for y in 0..max_y {
        print!("[");
        for x in 0..max_x {
            let val = ocean_floor[x][y];
            print!(
                "{}{}",
                if val > 0 {
                    val.to_string()
                } else {
                    " ".to_string()
                },
                if x == max_x - 1 { "" } else { "," }
            );
        }
        println!("]");
    }
}

fn count_at_least_twos(ocean_floor: &Vec<Vec<usize>>) -> usize {
    ocean_floor.iter().fold(0, |acc, col| {
        col.iter().fold(0, |acc, cell| match *cell {
            x if x >= 2 => acc + 1,
            _ => acc,
        }) + acc
    })
}

fn main() {
    let file_content = fs::read_to_string("2021/5/input.txt").expect("Cannot read input file");
    let segments = file_content
        .lines()
        .map(|line| {
            let segment = line
                .split(ARROW)
                .map(|cell| {
                    let values = cell
                        .split(',')
                        .map(|num| num.parse::<usize>().expect("Invalid number"))
                        .collect::<Vec<usize>>();
                    [values[0], values[1]]
                })
                .collect::<Vec<[usize; 2]>>();
            [segment[0], segment[1]]
        })
        .collect::<Vec<Segment>>();
    let max_x = segments.iter().fold(0, |max, s| max.max(s.max_x())) + 1;
    let max_y = segments.iter().fold(0, |max, s| max.max(s.max_y())) + 1;

    let mut ocean_floor = vec![vec![0usize; max_y]; max_x];

    segments
        .iter()
        .filter(|segment| segment.parallel_to_axles())
        .for_each(|s| {
            for (x, y) in s.iter() {
                ocean_floor[x][y] += 1
            }
        });

    print_solution(1, count_at_least_twos(&ocean_floor));

    segments
        .iter()
        .filter(|segment| !segment.parallel_to_axles())
        .for_each(|s| {
            for (x, y) in s.iter() {
                ocean_floor[x][y] += 1
            }
        });

    print_solution(2, count_at_least_twos(&ocean_floor));
}
