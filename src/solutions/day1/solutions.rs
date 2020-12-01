use crate::input::Input;
use crate::helpers;

pub struct Day1;

// See https://adventofcode.com/2020/day/1
impl Day1 {
    fn get_numbers(filename: &String) -> Vec<usize> {
        let wrapped_input = helpers::read_all::<usize>(&filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn solution1(filename: &String) {
        let numbers = Self::get_numbers(filename);

        'found: for i in 0..numbers.len() {
            let a = numbers[i];
            for j in i..numbers.len() {
                let b = numbers[j];

                if  a+b == 2020 {
                    println!("{}", a*b);
                    break 'found;
                }
            }
        }
    }

    fn solution2(filename: &String) {
        let numbers = Self::get_numbers(filename);

        // This won't scale very well as it's O(n^3). If an optimization
        // is needed look at the standard solution to the 3SUM
        // https://en.wikipedia.org/wiki/3SUM
        // The performance for me is good enough as n is small (0.003s)
        'found: for i in 0..numbers.len() {
            let a = numbers[i];
            for j in i..numbers.len() {
                let b = numbers[j];
                if a+b <= 2020 {
                    for k in j..numbers.len() {
                        let c = numbers[k];
                        if  a+b+c == 2020 {
                            println!("{}", a*b*c);
                            break 'found;
                        }
                    }
                }
            }
        }
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}

