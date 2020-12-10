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
        5 => day5::solutions::Day5::solution(&input),
        6 => day6::solutions::Day6::solution(&input),
        7 => day7::solutions::Day7::solution(&input),
        8 => day8::solutions::Day8::solution(&input),
        9 => day9::solutions::Day9::solution(&input),
        10 => day10::solutions::Day10::solution(&input),
        _ => println!("No solutions for day {}", &input.day)
    }
}
