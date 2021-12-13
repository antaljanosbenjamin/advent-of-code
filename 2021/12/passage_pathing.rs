use std::{
    collections::{HashMap, HashSet},
    fs,
};

use common::utility::print_solution;

fn is_full_uppercase(start: &str) -> bool {
    start.chars().all(|c| c.is_uppercase())
}

fn bfs(
    cave_infos: &HashMap<usize, (bool, Vec<usize>, &str)>,
    cave: usize,
    end: usize,
    visited: &mut HashSet<usize>,
) -> usize {
    if cave == end {
        return 1;
    }
    let cave_info = cave_infos.get(&cave).unwrap();
    if !cave_info.0 {
        visited.insert(cave);
    }
    let mut res = 0;
    for next_cave in &cave_info.1 {
        if visited.contains(next_cave) {
            continue;
        }
        res += bfs(&cave_infos, *next_cave, end, visited);
    }
    visited.remove(&cave);
    res
}

fn bfs2(
    cave_infos: &HashMap<usize, (bool, Vec<usize>, &str)>,
    cave: usize,
    start: usize,
    end: usize,
    visited: &mut HashSet<usize>,
    joker: Option<usize>,
) -> usize {
    let current_cave_is_joker = joker == Some(cave);
    if cave == end {
        return 1;
    }
    let cave_info = cave_infos.get(&cave).unwrap();
    if !cave_info.0 {
        visited.insert(cave);
    }
    let mut res = 0;
    for next_cave in &cave_info.1 {
        let mut new_joker = joker;
        if visited.contains(next_cave) {
            if new_joker.is_none() && *next_cave != start && *next_cave != end {
                new_joker = Some(*next_cave);
            } else {
                continue;
            }
        }
        res += bfs2(&cave_infos, *next_cave, start, end, visited, new_joker);
    }
    if !current_cave_is_joker {
        visited.remove(&cave);
    }
    res
}

fn part1(start: usize, end: usize, cave_infos: &HashMap<usize, (bool, Vec<usize>, &str)>) -> usize {
    bfs(cave_infos, start, end, &mut HashSet::new())
}

fn part1_unwinded(
    start: usize,
    end: usize,
    cave_infos: &HashMap<usize, (bool /*is_big*/, Vec<usize>, &str)>,
) -> usize {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut caves_to_visit = vec![start];
    let mut res = 1;
    while !caves_to_visit.is_empty() {
        let cave = caves_to_visit.pop().unwrap();
        let cave_info = cave_infos.get(&cave).unwrap();
        println!("Cave: {} ({})", cave_info.2, cave);
        if cave == end {
            res += 1;
            println!("\tend reached");
            continue;
        }

        if !cave_info.0 {
            println!("\tPushing cave to visited and stack");
            visited.insert(cave);
            stack.push(cave);
        }

        let mut next_caves = cave_info
            .1
            .iter()
            .filter(|c| !visited.contains(c))
            .map(|c| *c)
            .collect::<Vec<usize>>();

        println!("\tNext caves: {:?}", next_caves);
        if !next_caves.is_empty() {
            caves_to_visit.append(&mut next_caves);
            println!("\tContinue");
            continue;
        }

        println!("\tRemoving cave from visited and stack");
        let cave_to_return_from = stack.pop().unwrap();
        visited.remove(&cave_to_return_from);
    }
    res
}

fn part2(
    start: usize,
    end: usize,
    cave_infos: &HashMap<usize, (bool /*is_big*/, Vec<usize>, &str)>,
) -> usize {
    bfs2(cave_infos, start, start, end, &mut HashSet::new(), None)
}

fn main() {
    let file_content = fs::read_to_string("2021/12/input.txt").expect("Cannot read input file");
    let mut caves = HashMap::new();
    let mut counter: usize = 0;
    let mut get_cave_id = |cave| {
        if caves.contains_key(cave) {
            *caves.get(cave).unwrap()
        } else {
            counter += 1;
            caves.insert(cave, counter);
            counter
        }
    };

    let cave_infos: HashMap<usize, (bool, Vec<usize>, &str)> = file_content
        .lines()
        .map(|line| {
            let v = line.split('-').collect::<Vec<&str>>();
            (
                is_full_uppercase(v[0]),
                v[0],
                get_cave_id(v[0]),
                is_full_uppercase(v[1]),
                v[1],
                get_cave_id(v[1]),
            )
        })
        .fold(
            HashMap::new(),
            |mut cave_infos, (is_src_big, src_name, src, is_dst_big, dst_name, dst)| {
                let mut update_cave_info = |src, is_big, name, dst| {
                    let mut entry = cave_infos.entry(src).or_default();
                    entry.0 = is_big;
                    entry.1.push(dst);
                    entry.2 = name;
                };

                update_cave_info(src, is_src_big, src_name, dst);
                update_cave_info(dst, is_dst_big, dst_name, src);
                cave_infos
            },
        );

    let start = *caves.get("start").unwrap();
    let end = *caves.get("end").unwrap();
    print_solution(1, part1(start, end, &cave_infos));
    print_solution(1, part1_unwinded(start, end, &cave_infos));
    print_solution(2, part2(start, end, &cave_infos));
}
