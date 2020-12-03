use crate::helpers;
use crate::input::Input;
use crate::solutions::day2::password::{PasswordWithPolicy};

pub struct Day2;

// See https://adventofcode.com/2020/day/2
impl Day2 {
    fn get_passwords_and_policies(filename: &String) -> Vec<PasswordWithPolicy> {
        let wrapped_input = helpers::read_all::<PasswordWithPolicy>(&filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn solution1(filename: &String) {
        let passwords_and_policies = Self::get_passwords_and_policies(filename);
        let mut number_of_valid_passwords = 0;
        for password_with_policy in &passwords_and_policies {
            let password = &password_with_policy.password;
            let policy = &password_with_policy.policy;

            let mut count = 0;
            for letter in password.chars() {
                if letter == policy.letter {
                    count += 1
                }
            }

            let range = &password_with_policy.policy.range;
            if count >= range.min && count <= range.max {
                println!("Password is valid {}", password);
                number_of_valid_passwords += 1;
            }
        }

        println!("There were {} valid passwords.", number_of_valid_passwords);
    }

    fn solution2(filename: &String) {
        let passwords_and_policies = Self::get_passwords_and_policies(filename);
        let mut number_of_valid_passwords = 0;
        for password_with_policy in &passwords_and_policies {
            let password = &password_with_policy.password;
            let policy = &password_with_policy.policy;

            // Not really a range, but I'm to lazy to change the struct..
            let range = &password_with_policy.policy.range;
            let mut match_count = 0;
            for (i,letter) in password.chars().enumerate() {
                // println!("({},{})", i+1,letter);
                if (i + 1 == range.min || i + 1 == range.max) && letter == policy.letter {
                    match_count += 1
                }
            }

            // println!("Match count {}",match_count);
            if match_count == 1 {
                println!("Password is valid {}", password);
                number_of_valid_passwords += 1;
            }
        }

        println!("There were {} valid passwords.", number_of_valid_passwords);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
