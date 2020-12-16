use crate::helpers;
use crate::input::Input;
use crate::solutions::day12::ship::{Ship, Direction, NavigationSystem};

pub struct Day12;

// See https://adventofcode.com/2020/day/12
impl Day12 {
    fn get_directions(filename: &String) -> Vec<Direction> {
         helpers::read_all::<Direction>(&filename)
            .into_iter()
            .map(|direction| direction.unwrap())
            .collect()
    }


    fn solution1(filename: &String) {
        let directions = Self::get_directions(filename);
        let mut ship = Ship::new(NavigationSystem::Standard);

        for direction in directions {
            ship.process(direction);
        }

        println!("Distance: {}", ship.manhattan_distance());
    }

    fn solution2(filename: &String) {
        let directions = Self::get_directions(filename);
        let mut ship = Ship::new(NavigationSystem::WayPoint);

        for direction in directions {
            ship.process(direction);
        }

        println!("Distance: {}", ship.manhattan_distance());
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
