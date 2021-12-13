use std::{collections::HashMap, fs};

use common::utility::print_solution;

fn lanternfishes_after_days(
    fishes: &HashMap<usize /*days*/, usize /*count*/>,
    days: usize,
) -> usize {
    let mut today_fishes = fishes.clone();
    let mut tomorrow_fishes = HashMap::new();
    for _ in 0..days {
        today_fishes.iter().for_each(|(days, count)| match *days {
            x if x != 0 => {
                *tomorrow_fishes.entry(x - 1).or_default() += count;
            }
            0 => {
                *tomorrow_fishes.entry(6).or_default() += count;
                *tomorrow_fishes.entry(8).or_default() += count;
            }
            _ => {
                panic!("Unexpected value");
            }
        });

        std::mem::swap(&mut today_fishes, &mut tomorrow_fishes);
        tomorrow_fishes.clear();
    }
    today_fishes
        .iter()
        .fold(0, |acc, (_days, count)| acc + count)
}

fn main() {
    let file_content = fs::read_to_string("2021/6/input.txt").expect("Cannot read input file");
    let mut fishes: HashMap<usize /*days*/, usize /*count*/> = HashMap::new();
    file_content
        .split(',')
        .map(|str_num| str_num.parse::<usize>().expect("Invalid input"))
        .for_each(|days| *fishes.entry(days).or_default() += 1);

    print_solution(1, lanternfishes_after_days(&fishes, 80));
    print_solution(2, lanternfishes_after_days(&fishes, 256));
}
