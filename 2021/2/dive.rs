use common::utility::print_solution;
use std::fs;

use std::convert::TryFrom;

fn part1(commands: &Vec<(Direction, u32)>) -> (u32, u32) {
    commands
        .iter()
        .fold((0, 0), |pos, (direction, amount)| match direction {
            Direction::Up => (pos.0, pos.1 - amount),
            Direction::Forward => (pos.0 + amount, pos.1),
            Direction::Down => (pos.0, pos.1 + amount),
        })
}

fn part2(commands: &Vec<(Direction, u32)>) -> (u32, u32) {
    struct Values {
        horizontal_position: u32,
        depth: u32,
        aim: i32,
    }
    let values = commands.iter().fold(
        Values {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        },
        |mut values, (direction, amount)| {
            let amount_i32 = i32::try_from(*amount).expect("Too big amount");
            match direction {
                Direction::Up => {
                    // values.depth -= amount;
                    values.aim = values.aim - amount_i32;
                }
                Direction::Forward => {
                    values.horizontal_position += amount;
                    values.depth += u32::try_from(values.aim * amount_i32).expect("Negative depth");
                }
                Direction::Down => {
                    // values.depth += amount;
                    values.aim = values.aim + amount_i32;
                }
            }
            values
        },
    );
    (values.horizontal_position, values.depth)
}

enum Direction {
    Up,
    Forward,
    Down,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "up" => Ok(Direction::Up),
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Invalid direction string '{}'", string)),
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("2021/2/input.txt").expect("Cannot read input file");
    let commands: Vec<(Direction, u32)> = file_content
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .map(|(direction, amount)| {
            (
                Direction::try_from(direction).expect("Invalid direction"),
                amount.parse::<u32>().expect("Invalid amount"),
            )
        })
        .collect();

    let print_result = |part, (horizontal, depth)| print_solution(part, horizontal * depth);
    print_result(1, part1(&commands));
    print_result(2, part2(&commands));
}
