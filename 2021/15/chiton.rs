use std::{
    collections::{HashMap, HashSet},
    fs,
};

use common::utility::print_solution;

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

fn part1(risk_levels: &Vec<Vec<usize>>) -> usize {
    let height = risk_levels.len();
    let width = risk_levels[0].len();
    let mut next_moves = Vec::<(usize, (usize, usize))>::new();
    let mut known_points = HashMap::<(usize, usize), usize>::new();
    known_points.insert((0usize, 0usize), 0usize);
    for (r, c) in get_neighbor_fields(height, width, 0, 0) {
        next_moves.push((0 + risk_levels[r][c], (r, c)));
    }
    while !known_points.contains_key(&(height - 1, width - 1)) {
        next_moves.sort_by(|lhs, rhs| rhs.0.cmp(&lhs.0));
        let next_move = next_moves.pop().unwrap();

        if known_points.contains_key(&next_move.1) {
            continue;
        }
        known_points.insert(next_move.1, next_move.0);

        for (r, c) in get_neighbor_fields(height, width, next_move.1 .0, next_move.1 .1) {
            let move_to_add = (next_move.0 + risk_levels[r][c], (r, c));
            next_moves.push(move_to_add);
        }
    }

    *known_points.get(&(height - 1, width - 1)).unwrap()
}

fn part2(risk_levels: &Vec<Vec<usize>>) -> usize {
    const DUPLICATION_FACTOR: usize = 5;
    let original_height = risk_levels.len();
    let original_width = risk_levels[0].len();
    let height = original_height * DUPLICATION_FACTOR;
    let width = original_width * DUPLICATION_FACTOR;
    let mut new_risk_levels = vec![vec![0; width]; height];
    for r in 0..original_height {
        for c in 0..original_width {
            new_risk_levels[r][c] = risk_levels[r][c];
        }
    }
    for tile_r in 0..DUPLICATION_FACTOR {
        for tile_c in 0..DUPLICATION_FACTOR {
            if tile_r == 0 && tile_c == 0 {
                continue;
            }
            let (r_source_offset, c_source_offset) = if tile_r != 0 {
                ((tile_r - 1) * original_height, tile_c * original_width)
            } else {
                (tile_r * original_height, (tile_c - 1) * original_width)
            };
            let r_offset = tile_r * original_height;
            let c_offset = tile_c * original_width;
            for r in 0..original_height {
                for c in 0..original_width {
                    let r_source = r_source_offset + r;
                    let c_source = c_source_offset + c;
                    new_risk_levels[r_offset + r][c_offset + c] =
                        if 9 == new_risk_levels[r_source][c_source] {
                            1
                        } else {
                            new_risk_levels[r_source][c_source] + 1
                        };
                }
            }
        }
    }
    part1(&new_risk_levels)
}

fn main() {
    let file_content = fs::read_to_string("2021/15/input.txt").expect("Cannot read input file");
    let risk_levels = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("Invalid risk level")
                        .try_into()
                        .expect("Invalid risk level")
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    print_solution(1, part1(&risk_levels));
    print_solution(2, part2(&risk_levels));
}
