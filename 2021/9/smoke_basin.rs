use std::{collections::HashSet, fs};

use common::print_solution;

const NEIGHBOR_OFFSETS: [(i8, i8); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn get_neighbor_fields(height: usize, width: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    for offset in &NEIGHBOR_OFFSETS {
        let (r, c) = (
            row.wrapping_add(offset.0 as usize),
            col.wrapping_add(offset.1 as usize),
        );
        if r < height && c < width {
            neighbors.push((r, c));
        }
    }

    neighbors
}

#[allow(dead_code)]
fn print_basins(height: usize, width: usize, basins: &HashSet<(usize, usize)>) {
    for row in 0..height {
        for col in 0..width {
            let char = if basins.contains(&(row, col)) {
                'X'
            } else {
                'O'
            };
            print!("{}", char);
        }
        println!("");
    }
}

fn find_low_points(depths: &Vec<Vec<usize>>) -> HashSet<(usize, usize)> {
    let mut low_points = HashSet::<(usize, usize)>::new();
    let width = depths[0].len();
    let height = depths.len();
    for col in 0..width {
        for row in 0..height {
            let val = depths[row][col];
            if get_neighbor_fields(height, width, row, col)
                .iter()
                .all(|&(r, c)| depths[r][c] > val)
            {
                low_points.insert((row, col));
            }
        }
    }
    low_points
}

fn part1(depths: &Vec<Vec<usize>>, low_points: &HashSet<(usize, usize)>) -> usize {
    low_points
        .iter()
        .fold(0, |acc, &(r, c)| acc + depths[r][c] + 1)
}

fn part2(depths: &Vec<Vec<usize>>, low_points: &HashSet<(usize, usize)>) -> usize {
    let width = depths[0].len();
    let height = depths.len();
    let mut basin_sizes = Vec::new();
    let mut visited = HashSet::new();
    for basin in low_points {
        let mut size = 0;
        let mut coords_to_check = vec![(basin.0, basin.1, depths[basin.0][basin.1])];
        while !coords_to_check.is_empty() {
            let (r, c, expected) = coords_to_check.pop().unwrap();
            let value = depths[r][c];
            if value >= expected && !visited.contains(&(r, c)) && value != 9 {
                coords_to_check.append(
                    &mut get_neighbor_fields(height, width, r, c)
                        .into_iter()
                        .map(|(r, c)| (r, c, expected + 1))
                        .collect::<Vec<(usize, usize, usize)>>(),
                );
                size += 1;
                visited.insert((r, c));
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.sort_by(|lhs, rhs| rhs.cmp(lhs));
    basin_sizes.iter().take(3).fold(1, |acc, value| acc * value)
}

fn main() {
    let file_content = fs::read_to_string("2021/9/input.txt").expect("Cannot read input file");
    let depths = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Invalid depth")
                        .try_into()
                        .expect("Invalid depth")
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let low_points = find_low_points(&depths);
    print_solution(1, part1(&depths, &low_points));
    print_solution(2, part2(&depths, &low_points));
}
