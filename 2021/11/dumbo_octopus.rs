use std::{collections::HashSet, fs};

use common::print_solution;

const NEIGHBOR_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

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

fn step(octs: &mut Vec<Vec<usize>>) -> usize {
    let mut flashed = HashSet::new();
    let mut to_flash = Vec::new();
    let height = octs.len();
    let width = octs[0].len();

    for r in 0..height {
        for c in 0..width {
            let o = &mut octs[r][c];
            *o += 1;
            if *o > 9 {
                to_flash.push((r, c));
            }
        }
    }

    while !to_flash.is_empty() {
        let (r, c) = to_flash.pop().unwrap();
        for (nr, nc) in get_neighbor_fields(height, width, r, c) {
            let o = &mut octs[nr][nc];
            *o += 1;
            if *o == 10 && !flashed.contains(&(nr, nc)) {
                to_flash.push((nr, nc));
            }
        }
        flashed.insert((r, c));
    }
    flashed.iter().for_each(|&(r, c)| octs[r][c] = 0);
    flashed.len()
}

fn part1(octopuses: &Vec<Vec<usize>>) -> usize {
    let mut octs = octopuses.clone();
    let mut flash_count = 0;
    for _step in 0..100 {
        flash_count += step(&mut octs);
    }
    flash_count
}

fn part2(octopuses: &Vec<Vec<usize>>) -> usize {
    let mut octs = octopuses.clone();
    let height = octs.len();
    let width = octs[0].len();
    let max_flash_count = width * height;
    let mut step_counter = 0;
    loop {
        step_counter += 1;
        if step(&mut octs) == max_flash_count {
            return step_counter;
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("2021/11/input.txt").expect("Cannot read input file");
    let octopuses = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Invalid octopus energy")
                        .try_into()
                        .expect("Invalid octopus energy")
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    print_solution(1, part1(&octopuses));
    print_solution(2, part2(&octopuses));
}
