use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Index, Sub},
};

use common::utility::print_solution;

fn part1(inputs: &Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    inputs.iter().fold(0usize, |acc, (_signals, outputs)| {
        acc + outputs
            .iter()
            .filter(|&s| match s.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            })
            .count()
    })
}

fn part2(inputs: &Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    inputs.iter().fold(0usize, |acc, (signals, outputs)| {
        let numbers_and_signals = {
            let mut d2s = HashMap::new();
            signals.iter().for_each(|s| match s.len() {
                2 => {
                    d2s.insert(1, s);
                }
                3 => {
                    d2s.insert(7, s);
                }
                4 => {
                    d2s.insert(4, s);
                }
                7 => {
                    d2s.insert(8, s);
                }
                _ => (),
            });

            // 7 - 4 = a
            let a = d2s.index(&7).sub(d2s.index(&4));
            assert_eq!(a.len(), 1);

            // 1 is not subset of |6| = 6
            signals.iter().filter(|s| s.len() == 6).for_each(|s| {
                if !d2s.index(&1).is_subset(s) {
                    d2s.insert(6, s);
                }
            });

            // 1 - 6 = c
            let c = d2s.index(&1).sub(d2s.index(&6));
            assert_eq!(c.len(), 1);

            // 1 - c = f
            let f = d2s.index(&1).sub(&c);
            assert_eq!(f.len(), 1);

            // f is not subset of = 2
            signals.iter().filter(|s| !f.is_subset(s)).for_each(|s| {
                d2s.insert(2, s);
            });

            // 8 - 2 - f = b
            let b = d2s.index(&8).sub(d2s.index(&2)).sub(&f);
            assert_eq!(b.len(), 1);

            // c is not subset of |5| = 5
            signals
                .iter()
                .filter(|s| s.len() == 5 && !c.is_subset(s))
                .for_each(|s| {
                    d2s.insert(5, s);
                });

            // 2 - 5 - c = e
            let e = d2s.index(&2).sub(d2s.index(&5)).sub(&c);
            assert_eq!(e.len(), 1);

            // e is not subset of |6| = 9
            signals
                .iter()
                .filter(|s| s.len() == 6 && !e.is_subset(s))
                .for_each(|s| {
                    d2s.insert(9, s);
                });

            // c and f is subset of |5| = 3
            signals
                .iter()
                .filter(|s| s.len() == 5 && c.is_subset(s) && f.is_subset(s))
                .for_each(|s| {
                    d2s.insert(3, s);
                });

            // 4 - 7 - b = d
            let d = d2s.index(&4).sub(d2s.index(&7)).sub(&b);
            assert_eq!(d.len(), 1);

            // d is not subset of |6| = 0
            signals
                .iter()
                .filter(|s| s.len() == 6 && !d.is_subset(s))
                .for_each(|s| {
                    d2s.insert(0, s);
                });

            // 8 - 4 - 1 - b = g
            let g = d2s
                .index(&8)
                .sub(d2s.index(&7))
                .sub(d2s.index(&4))
                .sub(d2s.index(&1))
                .sub(&e);
            assert_eq!(g.len(), 1);
            d2s.into_iter().collect::<Vec<(usize, &HashSet<char>)>>()
        };

        acc + outputs.iter().fold(0, |acc, signals| {
            numbers_and_signals
                .iter()
                .filter(|(_num, sig)| sig.eq(&signals))
                .next()
                .expect("Unexpected output")
                .0
                + acc * 10
        })
    })
}

fn main() {
    let file_content = fs::read_to_string("2021/8/input.txt").expect("Cannot read input file");
    let inputs = file_content
        .lines()
        .map(|line| {
            let vec = line.split('|').collect::<Vec<&str>>();
            (
                vec[0]
                    .trim()
                    .split(' ')
                    .map(|s| s.chars().collect::<HashSet<char>>())
                    .collect::<Vec<HashSet<char>>>(),
                vec[1]
                    .trim()
                    .split(' ')
                    .map(|s| s.chars().collect::<HashSet<char>>())
                    .collect::<Vec<HashSet<char>>>(),
            )
        })
        .collect::<Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>>();

    print_solution(1, part1(&inputs));
    print_solution(2, part2(&inputs));
}
