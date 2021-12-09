use std::{collections::HashMap, fs};

use common::print_solution;

fn distance(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

fn min_max_pos(crabs: &HashMap</*pos*/ usize, /*count*/ usize>) -> (/*min*/ usize, /*max*/ usize) {
    (
        *crabs.keys().clone().min().unwrap(),
        *crabs.keys().max().unwrap(),
    )
}

fn get_total_fuel(
    crabs: &HashMap</*pos*/ usize, /*count*/ usize>,
    aligned_pos: usize,
    fuel_calc: fn(/*distance*/ usize) -> usize,
) -> usize {
    crabs.iter().fold(0, |total_fuel, (&pos, &count)| {
        fuel_calc(distance(pos, aligned_pos)) * count + total_fuel
    })
}

fn solve(
    crabs: &HashMap</*pos*/ usize, /*count*/ usize>,
    fuel_calc: fn(/*distance*/ usize) -> usize,
) -> usize {
    let (min_pos, max_pos) = min_max_pos(crabs);

    let mut min_total_fuel = get_total_fuel(&crabs, max_pos, fuel_calc);

    for aligned_pos in min_pos..max_pos {
        min_total_fuel = get_total_fuel(&crabs, aligned_pos, fuel_calc).min(min_total_fuel);
    }
    min_total_fuel
}

fn part1(crabs: &HashMap</*pos*/ usize, /*count*/ usize>) -> usize {
    solve(crabs, |distance| distance)
}

fn part2(crabs: &HashMap</*pos*/ usize, /*count*/ usize>) -> usize {
    solve(crabs, |distance| {
        let dist = distance as f64;
        ((dist as f64 + 1f64) / 2f64 * dist) as usize
    })
}

// Based on https://github.com/Lalica/Advent-of-Code/blob/master/AoC2021/solutions/day07.py
fn part1_better(crabs: &Vec<usize>) -> usize {
    let sorted_crabs = {
        let mut clone = crabs.clone();
        clone.sort();
        clone
    };

    let middle_index = (sorted_crabs.len() + 1) / 2;
    let middle_pos = sorted_crabs[middle_index];
    sorted_crabs
        .iter()
        .fold(0, |acc, &pos| distance(pos, middle_pos) + acc)
}

// Based on https://cdn.discordapp.com/attachments/541932275068174359/917782745894256640/crab-submarines.pdf
fn part2_better(crabs: &Vec<usize>) -> usize {
    let mean = crabs.iter().sum::<usize>() / crabs.len();
    let solve = |mean| {
        crabs
            .iter()
            .map(|pos| {
                let dist = distance(mean, *pos);
                (dist.pow(2) + dist + 1) / 2
            })
            .sum::<usize>()
    };
    solve(mean).min(solve(mean + 1))
}

fn main() {
    let file_content = fs::read_to_string("2021/7/input.txt").expect("Cannot read input file");
    let crabs_vec = file_content
        .split(',')
        .map(|str_num| str_num.parse::<usize>().expect("Invalid input"))
        .collect::<Vec<usize>>();

    let crabs = crabs_vec.iter().fold(
        HashMap::</*pos*/ usize, /*count*/ usize>::new(),
        |mut crabs, &pos| {
            *crabs.entry(pos).or_default() += 1;
            crabs
        },
    );

    print_solution(1, part1(&crabs));
    print_solution(1, part1_better(&crabs_vec));
    print_solution(2, part2(&crabs));
    print_solution(2, part2_better(&crabs_vec));
}
