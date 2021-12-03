use std::fmt::Display;

pub fn print_solution<T>(part_number: u32, solution: T)
where
    T: Display,
{
    println!("Part{} result is {}", part_number, solution);
}
