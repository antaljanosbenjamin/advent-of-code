use std::convert::TryInto;
use std::fs;

use common::print_solution;

fn count_ones(lines: &Vec<&str>) -> Vec<usize> {
    let bit_count = lines[0].len();

    lines
        .iter()
        .fold(vec![0usize; bit_count], |mut one_counts, line| {
            line.chars()
                .zip(one_counts.iter_mut())
                .for_each(|(char, counter)| {
                    if char == '1' {
                        *counter += 1
                    }
                });
            one_counts
        })
}

fn part1(line_count: usize, one_counts: &Vec<usize>) -> usize {
    let gamma_str = one_counts
        .iter()
        .map(|one_counts| {
            if *one_counts >= line_count / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    let gamma = usize::from_str_radix(&gamma_str, 2).expect("Wrong number");
    let max_value = 2usize.pow(one_counts.len().try_into().expect("Too much bits")) - 1;
    let epsilon = max_value - gamma;
    let solution = gamma * epsilon;

    solution
}

fn find_number_with_criteria(
    mut lines: Vec<&str>,
    mut one_counts: Vec<usize>,
    criteria_fn: fn(usize, &Vec<usize>, &Vec<&str>) -> bool,
) -> usize {
    if lines.len() == 1 {
        return usize::from_str_radix(lines.first().unwrap(), 2).expect("Invalid lines");
    }
    let bit_count = one_counts.len();
    for index in 0..bit_count {
        let mut new_lines = Vec::<&str>::new();
        let bit_criteria = if criteria_fn(index, &one_counts, &lines) {
            '1'
        } else {
            '0'
        };

        for line in &lines {
            if line.chars().nth(index).expect("Wrong input") == bit_criteria {
                new_lines.push(line);
                continue;
            }
            line.chars()
                .zip(one_counts.iter_mut())
                .for_each(|(char, count)| {
                    if char == '1' {
                        *count -= 1;
                    }
                });
        }
        std::mem::swap(&mut lines, &mut new_lines);
        new_lines.clear();
        if lines.len() == 1 {
            return usize::from_str_radix(lines.first().unwrap(), 2).expect("Invalid lines");
        }
    }
    unreachable!();
}

fn part2(lines: &Vec<&str>, one_counts: &Vec<usize>) -> usize {
    let oxygen_generator_rating = find_number_with_criteria(
        lines.clone(),
        one_counts.clone(),
        |index, one_counts, lines| one_counts[index] >= (lines.len() + 1) / 2,
    );
    let co2_scrubber_rating = find_number_with_criteria(
        lines.clone(),
        one_counts.clone(),
        |index, one_counts, lines| one_counts[index] < (lines.len() + 1) / 2,
    );
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let file_content = fs::read_to_string("2021/3/input.txt").expect("Cannot read input file");
    let lines = file_content.lines().map(|line| line).collect();
    let one_counts = count_ones(&lines);

    print_solution(1, part1(lines.len(), &one_counts));
    print_solution(1, part2(&lines, &one_counts));
}
