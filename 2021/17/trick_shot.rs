use std::fs;

use common::{
    coords::{Coords, CoordsImpl},
    utility::print_solution,
};

fn is_inside_rect(point: Coords<i64>, top_left: Coords<i64>, bottom_right: Coords<i64>) -> bool {
    point.x() >= top_left.x()
        && point.x() <= bottom_right.x()
        && point.y() <= top_left.y()
        && point.y() >= bottom_right.y()
}

fn throw(mut speed: Coords<i64>, top_left: Coords<i64>, bottom_right: Coords<i64>) -> (bool, i64) {
    let mut position = [0, 0];
    const DECREASE_X: Coords<i64> = [-1, 0];
    const DECREASE_Y: Coords<i64> = [0, -1];
    let mut highest_y = 0;
    loop {
        if is_inside_rect(position, top_left, bottom_right) {
            return (true, highest_y);
        }
        if position.x() > bottom_right.x() || position.y() < bottom_right.y() {
            return (false, highest_y);
        }
        highest_y = highest_y.max(position.y());
        position = position.add(&speed);
        if speed.x() > 0 {
            speed = speed.add(&DECREASE_X);
        }
        speed = speed.add(&DECREASE_Y);
    }
}

fn main() {
    let file_content = fs::read_to_string("2021/17/input.txt").expect("Cannot read input file");
    let mut boundary_iter = file_content.trim().split(' ').skip(2).map(|mut str| {
        str = &str[2..];
        if str.chars().rev().next().unwrap() == ',' {
            str = &str[0..str.len() - 1];
        }
        let mut coords_iter = str
            .split("..")
            .map(|n| n.parse::<i64>().expect("Unexpected input"));
        (
            coords_iter.next().expect("Unexpected input"),
            coords_iter.next().expect("Unexpected input"),
        )
    });

    let x_boundary = boundary_iter.next().expect("Unexpected input");
    let y_boundary = boundary_iter.next().expect("Unexpected input");

    let top_left = [x_boundary.0, y_boundary.1];
    let bottom_right = [x_boundary.1, y_boundary.0];

    let mut highest_y = 0;
    let mut vel = [0, 0];
    let mut count = 0;
    for x in 1..=x_boundary.1 {
        for y in y_boundary.0..y_boundary.1.abs() * 10 {
            let res = throw([x, y], top_left, bottom_right);
            if res.0 {
                count += 1;
                if res.1 > highest_y {
                    vel = [x, y];
                }
                highest_y = highest_y.max(res.1);
            }
        }
    }
    println!("{:?}", vel);
    print_solution(1, highest_y);
    print_solution(2, count);
}
