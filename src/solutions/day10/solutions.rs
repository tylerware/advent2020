use crate::input::Input;
use crate::helpers;
use num::bigint::BigInt;

pub struct Day10;

// See https://adventofcode.com/2020/day/10
impl Day10 {
    fn get_jolt_ratings(filename: &String) -> Vec<usize> {
        let wrapped_input = helpers::read_all::<usize>(&filename);
        let mut jolts = wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>();
        jolts.sort();
        jolts
    }


    fn solution1(filename: &String) {
        let starting_rating = 0;
        let mut jolt_rating = vec![starting_rating];
        jolt_rating.append(
            &mut Self::get_jolt_ratings(filename)
        );

        let device_rating = jolt_rating.last().unwrap() + 3;
        jolt_rating.push(device_rating);

        let mut diff3 = 0;
        let mut diff1 = 0;
        for i in 0..(jolt_rating.len() - 1) {
            let a = jolt_rating[i];
            let b = jolt_rating[i + 1];
            let diff = b - a;

            if diff == 1 {
                diff1 += 1;
            }
            else if diff == 3 {
                diff3 += 1;
            }
            else if diff != 2 {
                // sanity check..
                println!("Adapter difference isn't 1, 2 or 3. There is an issue with the data.");
            }

        }

        println!("Computing multiple of the number of 1 and 3 jolt differences.");
        println!("{}*{} = {}", diff1, diff3, diff1 * diff3);
    }

    fn solution2(filename: &String) {
        let starting_rating = 0;
        let mut jolt_rating = vec![starting_rating];
        jolt_rating.append(
            &mut Self::get_jolt_ratings(filename)
        );

        let mut ways_to_node = vec![BigInt::from(0); jolt_rating.len()];
        // There is one 'way' to 1.. mostly we are just saying that it starts at 1
        // because it makes the math below work :)
        ways_to_node[0] = BigInt::from(1);

        for i in 0..(jolt_rating.len() - 1) {
            let jolt_rating_i = jolt_rating[i];
            let ways_to_i = ways_to_node[i].clone();

            'devices_following: for j in (i + 1)..jolt_rating.len() {
                let jolt_rating_j = jolt_rating[j];
                let diff = jolt_rating_j - jolt_rating_i;
                if diff >= 1 && diff <= 3 {
                    ways_to_node[j] += &ways_to_i;
                }
                else {
                    break 'devices_following;
                }
            }
        }

        println!("Total configurations: {}", ways_to_node.last().unwrap());
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}

