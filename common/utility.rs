use std::fmt::Display;

pub fn print_solution<T>(part_number: u32, solution: T)
where
    T: Display,
{
    println!("Part{} result is {}", part_number, solution);
}

pub fn distance<T>(lhs: T, rhs: T) -> T
where
    T: std::cmp::PartialOrd,
    T: std::ops::Sub<Output = T>,
{
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}
