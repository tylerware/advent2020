use crate::input::Input;
use crate::helpers;

pub struct Day9;

// See https://adventofcode.com/2020/day/9
impl Day9 {
    fn get_numbers(filename: &String) -> Vec<usize> {
        let wrapped_input = helpers::read_all::<usize>(&filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn find_violator(numbers: &Vec<usize>) -> usize {
        let mut violator: usize = 0;
        'found_violator: for i in 25..numbers.len() {
            let a = numbers[i];

            let mut matching_sum = false;
            'found_sum: for j in (i - 25)..(i - 1) {
                let b = numbers[j];

                for k in (j + 1)..i {
                    let c = numbers[k];

                    if  b+c == a {
                        matching_sum = true;
                        break 'found_sum;
                    }
                }
            }

            if !matching_sum {
                violator = a;
                break 'found_violator;
            }
        }

        violator
    }

    fn solution1(filename: &String) {
        let numbers = Self::get_numbers(filename);
        let violator = Self::find_violator(&numbers);
        println!("Violator: {}", violator);
    }

    fn solution2(filename: &String) {
        let numbers = Self::get_numbers(filename);
        let violator = Self::find_violator(&numbers);

        'range_found: for i in 0..numbers.len() {
            let term = numbers[i];
            let mut smallest = term;
            let mut largest = term;
            let mut partial_sum = term;

            let mut j = i + 1;
            while partial_sum < violator {
                let term = numbers[j];
                if term < smallest {
                    smallest = term;
                }
                else if term > largest {
                    largest = term;
                }

                partial_sum += term;

                j += 1;
            }

            if partial_sum == violator {
                println!("{} + {} = {}", smallest, largest, smallest + largest);
                break 'range_found;
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

