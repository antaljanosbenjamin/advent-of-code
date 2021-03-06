use std::{collections::HashMap, fs};

use common::utility::print_solution;

const ARROW: &'static str = " -> ";

fn replace_one_step(formula: &str, insertion_pairs: &HashMap<(char, char), char>) -> String {
    let mut result = vec!['_'; formula.len() * 2];
    result[0] = formula.chars().nth(0).expect("Unexpected formula");
    let mut counter = 1;
    formula
        .chars()
        .zip(formula.chars().skip(1))
        .for_each(|(first, second)| {
            if let Some(replacement) = insertion_pairs.get(&(first, second)) {
                result[counter] = *replacement;
                counter += 1;
            }
            result[counter] = second;
            counter += 1;
        });
    result
        .into_iter()
        .take_while(|c| *c != '_')
        .collect::<String>()
}

const PART1_STEPS: usize = 10;
const PART2_STEPS: usize = 40;

// Naive method
fn part1(formula_str: &str, insertion_pairs: &HashMap<(char, char), char>) -> usize {
    let mut formula = formula_str.to_string();
    for _i in 0..PART1_STEPS {
        formula = replace_one_step(&formula, &insertion_pairs);
    }

    let counters: HashMap<char, usize> = formula.chars().fold(HashMap::new(), |mut counters, c| {
        *counters.entry(c).or_default() += 1;
        counters
    });

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();
    max - min
}

fn count_n_step(
    first: char,
    second: char,
    n: usize,
    insertion_pairs: &HashMap<(char, char), char>,
    counters: &mut HashMap<char, usize>,
    cache: &HashMap<(char, char), HashMap<char, usize>>,
) {
    if n == 0 {
        if let Some(cached) = cache.get(&(first, second)) {
            cached
                .iter()
                .for_each(|(c, n)| *counters.entry(*c).or_default() += n);
        }
        return;
    }

    if let Some(new_char) = insertion_pairs.get(&(first, second)) {
        *counters.entry(*new_char).or_default() += 1;
        count_n_step(first, *new_char, n - 1, insertion_pairs, counters, cache);
        count_n_step(*new_char, second, n - 1, insertion_pairs, counters, cache);
    }
}

//  I have the POOOOOOWEEEEEEEER!
fn part2(formula_str: &str, insertion_pairs: &HashMap<(char, char), char>) -> usize {
    let mut cache = HashMap::<(char, char), HashMap<char, usize>>::new();
    let empty_cache = HashMap::<(char, char), HashMap<char, usize>>::new();
    let steps = 20;
    for &(a, b) in insertion_pairs.keys() {
        let mut counters = HashMap::<char, usize>::new();
        count_n_step(a, b, steps, &insertion_pairs, &mut counters, &empty_cache);
        cache.insert((a, b), counters);
    }

    let mut counters = HashMap::<char, usize>::new();
    formula_str
        .chars()
        .for_each(|c| *counters.entry(c).or_default() += 1);
    formula_str
        .chars()
        .zip(formula_str.chars().skip(1))
        .for_each(|(a, b)| count_n_step(a, b, steps, &insertion_pairs, &mut counters, &cache));

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();
    max - min
}

fn step_one(
    pairs: &HashMap<(char, char), usize>,
    insertion_pairs: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    pairs
        .iter()
        .fold(HashMap::new(), |mut acc, (&(a, b), &count)| {
            if let Some(&new_char) = insertion_pairs.get(&(a, b)) {
                *acc.entry((a, new_char)).or_default() += count;
                *acc.entry((new_char, b)).or_default() += count;
            }
            acc
        })
}

// After opening the subreddit and seeing the lanternfish PTSD posts, it popped...
fn count_n_step_efficient(
    formula_str: &str,
    insertion_pairs: &HashMap<(char, char), char>,
    steps: usize,
) -> usize {
    let mut pairs: HashMap<(char, char), usize> = formula_str
        .chars()
        .zip(formula_str.chars().skip(1))
        .fold(HashMap::new(), |mut acc, (a, b)| {
            *acc.entry((a, b)).or_default() += 1;
            acc
        });

    for _i in 0..steps {
        pairs = step_one(&pairs, insertion_pairs);
    }

    let mut counters = pairs.iter().fold(
        HashMap::<char, usize>::new(),
        |mut acc, (&(a, b), &count)| {
            *acc.entry(a).or_default() += count;
            *acc.entry(b).or_default() += count;
            acc
        },
    );

    *counters
        .entry(formula_str.chars().next().unwrap())
        .or_default() += 1;
    *counters
        .entry(formula_str.chars().rev().next().unwrap())
        .or_default() += 1;
    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();
    (max - min) / 2
}

fn part1_efficient(formula_str: &str, insertion_pairs: &HashMap<(char, char), char>) -> usize {
    count_n_step_efficient(formula_str, insertion_pairs, PART1_STEPS)
}

fn part2_efficient(formula_str: &str, insertion_pairs: &HashMap<(char, char), char>) -> usize {
    count_n_step_efficient(formula_str, insertion_pairs, PART2_STEPS)
}

fn main() {
    let file_content =
        fs::read_to_string("2021/14/input_small.txt").expect("Cannot read input file");
    let insertion_pairs = file_content
        .lines()
        .skip(2)
        .map(|line| {
            let insertion_pair = line.split(ARROW).collect::<Vec<&str>>();
            (
                (
                    insertion_pair[0].chars().nth(0).expect("Unexpected input"),
                    insertion_pair[0].chars().nth(1).expect("Unexpected input"),
                ),
                insertion_pair[1].chars().nth(0).expect("Unexpected input"),
            )
        })
        .fold(HashMap::new(), |mut insertion_pairs, (base, insertion)| {
            insertion_pairs.insert(base, insertion);
            insertion_pairs
        });
    let formula = file_content
        .lines()
        .next()
        .expect("Unexpected input")
        .trim();

    print_solution(1, part1_efficient(formula, &insertion_pairs));
    print_solution(1, part1(formula, &insertion_pairs));
    print_solution(2, part2_efficient(formula, &insertion_pairs));
    print_solution(2, part2(formula, &insertion_pairs));
}
