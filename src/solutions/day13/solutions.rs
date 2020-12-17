use crate::input::Input;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::str::FromStr;
use std::num::ParseIntError;
use num::bigint::BigInt;
use num::ToPrimitive;

pub struct Day13;

// See https://adventofcode.com/2020/day/13
impl Day13 {
    pub fn get_estimate_and_lines(filename: &String) -> (usize, Vec<Result<usize, ParseIntError>>) {
        let buffered = BufReader::new(File::open(filename).unwrap());
        let lines: Vec<String> = buffered.lines()
                            .map(|line| line.unwrap())
                            .collect();

        let arrival_estimate = usize::from_str(&lines[0]).unwrap();
        let bus_lines = lines[1].split(',')
                                .map(|x| usize::from_str(x))
                                .collect();

        (arrival_estimate, bus_lines)
    }


    fn solution1(filename: &String) {
        let (estimate, bus_lines) = Self::get_estimate_and_lines(filename);
        let (buss_id, wait) = bus_lines.into_iter()
                                       .filter_map(|x| x.ok())
                                       .map(|buss_id| {
                                           // buss's next arrival
                                           (buss_id, buss_id - (estimate % buss_id))
                                       })
                                       .min_by(|x,y| x.1.cmp(&y.1))
                                       .unwrap();

        println!("Your buss ({}) will arrive in {}.", buss_id, wait);
        println!("Answer: {}", buss_id*wait);
    }

    fn solution2(filename: &String) {
        let (_estimate, bus_lines) = Self::get_estimate_and_lines(filename);
        let mut ids_with_mod: Vec<(usize, usize)> = bus_lines.into_iter()
                       .enumerate()
                       .filter(|x| x.1.is_ok())
                       .map(|(i, buss_id)| {
                           // We'll be using the Chinese Remainder Theorem
                           let a = -(i as i32);
                           let b = buss_id.unwrap() as i32;

                           // Finding the 'true' mod
                           let a_mod_b = ((a % b) + b) % b;
                           (a_mod_b as usize, b as usize)
                       })
                       .collect();

        ids_with_mod.sort_by(|x,y| y.1.cmp(&x.1));

        println!("{:?}", ids_with_mod);
        let mut iter =  ids_with_mod.iter();
        let mut current_buss = iter.next().unwrap();
        let mut a = BigInt::from(current_buss.0);
        let mut b = BigInt::from(current_buss.1);
        let mut i = BigInt::from(0);
        current_buss = iter.next().unwrap();
        let t: BigInt;
        loop {
            let j = &a + &i*&b;
            if (&j % current_buss.1).to_usize().unwrap() == current_buss.0 {
                let next_el = iter.next();
                b *= current_buss.1;
                println!("j = {}", j);
                match next_el {
                    Some(x) => {
                        a = j;
                        i = BigInt::from(0);
                        current_buss = x;
                    },
                    None => {
                        t = j;
                        break;
                    }
                }
            }
            else {
                i += 1;
            }
        }



        let (_estimate, bus_lines) = Self::get_estimate_and_lines(filename);
        let b_check: Vec<(usize,usize)> = bus_lines.into_iter()
                 .enumerate()
                 .filter(|x| x.1.is_ok())
                 .map(|x| (x.0, x.1.unwrap()))
                 .collect();
        println!("{:?}", b_check);
        let all_fit = b_check.iter()
                             .all(|(i, id)| {
                                 if ((&t + i) % id).to_usize().unwrap() == 0 {
                                     true
                                 }
                                 else {
                                     println!("Failed ({},{})", i, id);
                                     false
                                 }
                             });
        println!("Does {} work for all? {}", t, match all_fit {
            true => "Yes.",
            false => "No."
        });
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
