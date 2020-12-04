use crate::helpers;
use crate::input::Input;
use crate::solutions::day4::credentials::Credential;
use std::fs::File;
use std::io::{BufReader,Write,BufRead,LineWriter};

pub struct Day4;

// See https://adventofcode.com/2020/day/4
impl Day4 {

    // Make the batches all be on one line
    pub fn normalize_file(filename: &String) -> String {
        let buffered = BufReader::new(File::open(filename).unwrap());
        let mut normalized_filename = filename.to_string();
        normalized_filename.push_str(".norm");

        let normalized_file = File::create(&normalized_filename).unwrap();
        let mut file = LineWriter::new(&normalized_file);

        let lines = buffered.lines()
                            .filter_map(|line| line.ok());
        for line in lines {
            if line == "" {
                file.write_all(b"\n").unwrap();
            }
            else {
                file.write_all(b" ").unwrap();
                file.write_all(line.as_bytes()).unwrap();
            }
        }

        file.flush().unwrap();
        normalized_filename
    }

    fn get_credentials(filename: &String) -> Vec<Credential> {
       let normalized_filename = Self::normalize_file(filename);
        let wrapped_input = helpers::read_all::<Credential>(&normalized_filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn solution1(filename: &String) {
        let creds = Self::get_credentials(filename);
        let mut count_valid = 0;
        for cred in creds.iter() {
            if cred.has_north_pole_credential_fields() {
               count_valid += 1;
            }
        }

        println!("Number of valid passports: {}", count_valid);
    }

    fn solution2(filename: &String) {
        let creds = Self::get_credentials(filename);
        let mut count_valid = 0;
        for cred in creds.iter() {
            if cred.is_valid_north_pole_credential() {
               count_valid += 1;
            }
        }

        println!("Number of valid passports: {}", count_valid);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
