mod cli;
mod input;
mod solutions;
mod helpers;

use solutions::*;
fn main() {
    let input = cli::parse_input();
    println!("Running solution for day {} challenge {} with input file {}.", &input.day, &input.challenge, &input.filename);
    match &input.day {
        1 => day1::solutions::Day1::solution(&input),
        2 => day2::solutions::Day2::solution(&input),
        3 => day3::solutions::Day3::solution(&input),
        4 => day4::solutions::Day4::solution(&input),
        _ => println!("No solutions for day {}", &input.day)
    }
}
