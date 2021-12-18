use std::fs;

use common::utility::print_solution;

#[derive(Clone, Copy, Debug)]
enum Number {
    Empty,
    Single(u32),
    Pair,
}

impl Number {
    fn is_empty(&self) -> bool {
        match self {
            Number::Empty => true,
            _ => false,
        }
    }
    fn is_pair(&self) -> bool {
        match self {
            Number::Pair => true,
            _ => false,
        }
    }
}

// from https://users.rust-lang.org/t/logarithm-of-integers/8506/5
const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn log_2(x: usize) -> u32 {
    num_bits::<usize>() as u32 - x.leading_zeros() - 1
}

fn get_left(index: usize) -> usize {
    index * 2 + 1
}

fn get_right(index: usize) -> usize {
    index * 2 + 2
}

fn get_parent(index: usize) -> usize {
    (index - 1) / 2
}

fn add_to_side(
    nums: &mut [Number; 64],
    index: usize,
    uncle_selector: fn(usize) -> usize,
    child_selector: fn(usize) -> usize,
) {
    let value = match nums[index] {
        Number::Single(value) => value,
        _ => panic!("Unexpected value in explode"),
    };
    let mut ancestor = get_parent(index);
    let mut previous_ancestor = index;
    while ancestor > 0 && uncle_selector(ancestor) == previous_ancestor {
        previous_ancestor = ancestor;
        ancestor = get_parent(ancestor);
    }

    let uncle = uncle_selector(ancestor);
    if uncle == previous_ancestor {
        return;
    }

    let mut target = uncle;
    while nums[target].is_pair() {
        target = child_selector(target);
    }
    match &mut nums[target] {
        Number::Single(uncle_value) => *uncle_value += value,
        _ => panic!("Unexpected value in explode"),
    }
}

fn explode(nums: &mut [Number; 64], index: usize) -> bool {
    match nums[index] {
        Number::Empty => panic!("Empty during explode!"),
        Number::Single(_) => false,
        Number::Pair => {
            let level = log_2(index + 1);
            if level < 4 {
                explode(nums, get_left(index)) || explode(nums, get_right(index))
            } else {
                add_to_side(nums, get_left(index), get_left, get_right);
                add_to_side(nums, get_right(index), get_right, get_left);
                nums[index] = Number::Single(0);
                true
            }
        }
    }
}

fn split(nums: &mut [Number; 64], index: usize) -> bool {
    match nums[index] {
        Number::Empty => panic!("Empty during explode!"),
        Number::Single(value) if value < 10 => false,
        Number::Single(value) => {
            let left_value = value / 2;
            let right_value = (value + 1) / 2;
            let left_index = get_left(index);
            let right_index = get_right(index);
            nums[index] = Number::Pair;
            nums[left_index] = Number::Single(left_value);
            nums[right_index] = Number::Single(right_value);
            true
        }
        Number::Pair => {
            if !split(nums, get_left(index)) {
                split(nums, get_right(index))
            } else {
                true
            }
        }
    }
}

fn parse_single_number(c: char) -> Number {
    Number::Single(c.to_digit(10).unwrap())
}

fn parse_next_number(str: &[char], nums: &mut [Number; 64], index: usize) -> usize {
    if str[0] == '[' {
        nums[index] = Number::Pair;
        let mut consumed = 1;
        consumed += parse_next_number(&str[consumed..], nums, get_left(index));
        consumed += 1; // comma
        consumed += parse_next_number(&str[consumed..], nums, get_right(index));
        consumed += 1; // closing ]
        consumed
    } else {
        nums[index] = parse_single_number(str[0]);
        1
    }
}

#[allow(dead_code)]
fn print_num(nums: &[Number; 64], index: usize) {
    match nums[index] {
        Number::Single(value) => {
            print!("{}", value);
        }
        Number::Pair => {
            let left = get_left(index);
            let right = get_right(index);
            print!("[");
            print_num(nums, left);
            print!(",");
            print_num(nums, right);
            print!("]");
        }
        Number::Empty => panic!("Unexpected value while printing"),
    }
}

fn copy_num_rec(from: &[Number; 64], from_index: usize, to: &mut [Number; 64], to_index: usize) {
    if from[from_index].is_empty() {
        return;
    }

    to[to_index] = from[from_index];

    if from[from_index].is_pair() {
        copy_num_rec(from, get_left(from_index), to, get_left(to_index));
        copy_num_rec(from, get_right(from_index), to, get_right(to_index));
    }
}

fn copy_num(from: &[Number; 64], to: &mut [Number; 64], start_index: usize) {
    copy_num_rec(from, 0, to, start_index);
}

fn add(lhs_nums: &[Number; 64], rhs_nums: &[Number; 64]) -> [Number; 64] {
    let mut result = [Number::Empty; 64];
    let left_index = get_left(0);
    let right_index = get_right(0);
    result[0] = Number::Pair;
    copy_num(lhs_nums, &mut result, left_index);
    copy_num(rhs_nums, &mut result, right_index);
    let explode_or_split = |nums: &mut [Number; 64]| {
        if explode(nums, 0) {
            true
        } else {
            split(nums, 0)
        }
    };
    while explode_or_split(&mut result) {}
    result
}

fn magnitude(nums: &[Number; 64], index: usize) -> u32 {
    match nums[index] {
        Number::Empty => panic!("Empty during magnitude calculatio!"),
        Number::Single(value) => value,
        Number::Pair => {
            3 * magnitude(nums, get_left(index)) + 2 * magnitude(nums, get_right(index))
        }
    }
}

fn main() {
    let file_content = fs::read_to_string("2021/18/input.txt")
        .expect("Cannot read input file")
        .trim()
        .to_string();

    let nums_array = file_content
        .lines()
        .map(|l| {
            let mut nums = [Number::Empty; 64];
            let consumed = parse_next_number(&l.chars().collect::<Vec<char>>(), &mut nums, 0);
            assert_eq!(consumed, l.len());
            nums
        })
        .collect::<Vec<[Number; 64]>>();
    let part1 = nums_array
        .iter()
        .skip(2)
        .fold(add(&nums_array[0], &nums_array[1]), |acc, nums| {
            add(&acc, nums)
        });

    print_solution(1, magnitude(&part1, 0));

    let mut maximum_magnitude = 0;
    for lhs_index in 0..nums_array.len() {
        for rhs_index in 0..nums_array.len() {
            if lhs_index != rhs_index {
                maximum_magnitude = maximum_magnitude.max(magnitude(
                    &add(&nums_array[lhs_index], &nums_array[rhs_index]),
                    0,
                ));
            }
        }
    }
    print_solution(2, maximum_magnitude);
}
