mod cli;
mod input;
mod solutions;
mod helpers;

use solutions::{day1};
fn main() {
    let input = cli::parse_input();
    println!("Running solution for day {} challenge {} with input file {}.", &input.day, &input.challenge, &input.filename);
    match &input.day {
        1 => day1::solutions::Day1::solution(&input),
        _ => println!("No solutions for day {}", &input.day)
    }
}
