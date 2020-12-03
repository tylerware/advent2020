use crate::helpers;
use crate::input::Input;
use crate::solutions::day3::map::{TreeLine,TreeMap,Slope};

pub struct Day3;

// See https://adventofcode.com/2020/day/3
impl Day3 {
    fn get_tree_map(filename: &String) -> TreeMap {
        let wrapped_input = helpers::read_all::<TreeLine>(&filename);
        let tree_lines = wrapped_input.into_iter()
                        .map(|n| n.unwrap())
                        .collect::<Vec<_>>();
       TreeMap::new(tree_lines)
    }

    fn solution1(filename: &String) {
        let tree_map = Self::get_tree_map(filename);
        let tree_count = tree_map.count_trees_on_slope(Slope {
            down: 1,
            right: 3
        });
        println!("Number of trees we'll hit: {} (ouch!)", tree_count);
    }

    fn solution2(filename: &String) {
        let tree_map = Self::get_tree_map(filename);
        let slopes: Vec<Slope> = vec![
            Slope { down: 1, right: 1 },
            Slope { down: 1, right: 3 },
            Slope { down: 1, right: 5 },
            Slope { down: 1, right: 7 },
            Slope { down: 2, right: 1 },
        ];

        let mut product = 1;
        for slope in slopes.into_iter() {
            println!("Computing slope: ({}, {})", &slope.right, &slope.down);
            let tree_count = tree_map.count_trees_on_slope(slope);
            println!("Number of trees we'll hit: {} (ouch!)", tree_count);
            product *= tree_count;
        }

        println!("Product: {}", product);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
