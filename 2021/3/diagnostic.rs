use common::print_solution;
use std::convert::TryInto;
use std::fs;

fn part1(lines: &Vec<&str>) -> usize {
    let bit_count = lines[0].len();

    let (line_count, one_counts) = lines.iter().fold(
        (0u32, vec![0u32; bit_count]),
        |(line_count, mut one_counts), line| {
            line.chars()
                .enumerate()
                .filter(|(_index, char)| *char == '1')
                .for_each(|(index, _char)| one_counts[index] += 1);
            (line_count + 1, one_counts)
        },
    );

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
    let max_value = 2usize.pow(bit_count.try_into().unwrap()) - 1;
    let epsilon = max_value - gamma;
    let solution = gamma * epsilon;

    solution
}

fn main() {
    let file_content = fs::read_to_string("2021/3/input.txt").expect("Cannot read input file");
    let lines = file_content.lines().map(|line| line).collect();

    print_solution(1, part1(&lines));
}
