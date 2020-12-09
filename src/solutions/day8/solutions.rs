use crate::helpers;
use crate::input::Input;
use crate::solutions::day8::operations::{Operation,OperationType};

pub struct Day8;

// See https://adventofcode.com/2020/day/8
impl Day8 {

    fn get_operations(filename: &String) -> Vec<Operation> {
        let wrapped_input = helpers::read_all::<Operation>(&filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn run_operations(operations: &Vec<Operation>) -> bool {
        let mut visited_locations: Vec<usize> = vec![];
        let mut location: i32 = 0;
        let mut accumulator = 0;
        let mut terminated = false;

        'run_loop: loop {
            let operation = operations.get(location as usize).unwrap();
            let mut inc = 1;

            if visited_locations.contains(&(location as usize)) {
                break 'run_loop;
            }
            visited_locations.push(location as usize);

            match operation.op_type {
                OperationType::Accumulator => {
                    accumulator += operation.arg;
                },
                OperationType::Jump => {
                    inc = operation.arg;
                },
               _ => ()
            }

            location += inc;
            if location < 0 || (location as usize) >= operations.len() {
                if (location as usize) >= operations.len() {
                    println!("Current loc: {}", location);
                    terminated = true;
                }
                break 'run_loop;
            }
        }
        println!("Final value of accumulator: {}", accumulator);
        terminated
    }

    fn solution1(filename: &String) {
        let operations = Self::get_operations(filename);
        Self::run_operations(&operations);
    }

    fn solution2(filename: &String) {
        let operations = Self::get_operations(filename);
        'loop_fix: for (i, operation) in operations.iter().enumerate() {
            match operation.op_type {
                OperationType::Accumulator => (),
                _ => {
                    let new_op_type: OperationType;
                    match operation.op_type {
                        OperationType::Jump => new_op_type = OperationType::NoOperation,
                        OperationType::NoOperation | _ => new_op_type = OperationType::Jump
                    }

                    let mut revised_operations: Vec<Operation> = operations.iter().copied().collect();
                    revised_operations.get_mut(i).unwrap().op_type = new_op_type;

                    let run_result = Self::run_operations(&revised_operations);
                    if run_result {
                        break 'loop_fix
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
