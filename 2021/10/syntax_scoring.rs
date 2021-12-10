use std::fs;

use common::print_solution;

fn are_pairs(lhs: char, rhs: char) -> bool {
    match (lhs, rhs) {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn main() {
    let file_content = fs::read_to_string("2021/10/input.txt").expect("Cannot read input file");
    let lines = file_content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (part1, mut part2) = lines.iter().fold((0i64, Vec::new()), |mut acc, line| {
        let mut stack = Vec::new();
        for c in line {
            match c {
                '(' | '{' | '[' | '<' => {
                    stack.push(*c);
                }
                ')' | '}' | ']' | '>' => {
                    let opening = stack.pop().unwrap();
                    if !are_pairs(opening, *c) {
                        acc.0 += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Unexpected char"),
                        };
                        stack.clear();
                        break;
                    }
                }
                _ => panic!("Unexpected char"),
            };
        }
        if !stack.is_empty() {
            let autocomplete_score: i64 = stack
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '{' => 3,
                    '[' => 2,
                    '<' => 4,
                    _ => panic!("Unexpected char"),
                })
                .fold(0, |acc, p| acc * 5 + p);

            acc.1.push(autocomplete_score);
        }
        acc
    });
    part2.sort();

    print_solution(1, part1);
    print_solution(2, part2[part2.len() / 2]);
}
