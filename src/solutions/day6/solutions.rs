use crate::input::Input;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;

pub struct Day6;

// See https://adventofcode.com/2020/day/6
impl Day6 {
    pub fn get_answers_by_group(filename: &String) -> Vec<Vec<String>> {
        let buffered = BufReader::new(File::open(filename).unwrap());
        let lines = buffered.lines()
                            .filter_map(|line| line.ok());

        let mut answers_by_group: Vec<Vec<String>> = vec![];
        let mut group: Vec<String> = vec![];
        for line in lines {

            if line == "" {
                answers_by_group.push(group);
                group = vec![];
            }
            else {
                group.push(line);
            }
        }

        if group.len() > 0 {
           answers_by_group.push(group)
        }

        answers_by_group
    }

    fn get_answers(filename: &String) -> Vec<String> {
        let answers_by_group = Self::get_answers_by_group(&filename);
        answers_by_group.iter()
                        .map(|answers| {
                            answers.iter()
                                   .fold(String::new(), |carry, x| {
                                       let mut concat_answer = "".to_string();
                                       concat_answer.push_str(&carry);
                                       concat_answer.push_str(x);

                                       let mut split_answers = concat_answer
                                           .chars()
                                           .clone()
                                           .collect::<Vec<char>>();

                                       split_answers.sort();
                                       split_answers.dedup();
                                       split_answers.into_iter().collect::<String>()
                                   })
                        })
            .collect()

    }

    fn solution1(filename: &String) {
        let groups_answers = Self::get_answers(filename);
        let yes_count = groups_answers.iter()
                                .map(|answers| answers.len())
                                .fold(0, |carry, x| carry + x);
        println!("Yes counts: {}", yes_count);
    }

    fn solution2(filename: &String) {
        let answers = Self::get_answers_by_group(filename);
        let yes_count = answers.iter()
                                // For every group get the count for all questions answered by their members
                               .map(|group| {
                                   let group_size = group.len();

                                   // record number of answers per question
                                   let mut answer_counts: HashMap<char, usize> = HashMap::new();
                                   group.iter()
                                        .for_each(|answer| {
                                            answer.chars()
                                                  .for_each(|c| {
                                                      let mut count: usize = 0;
                                                      if answer_counts.contains_key(&c) {
                                                          count = *answer_counts.get(&c).unwrap();
                                                      }

                                                      count += 1;
                                                      answer_counts.insert(c, count);
                                                  })
                                        });

                                   // add up the number of questions that every answered
                                   let all_answered_count = answer_counts.iter()
                                                                         .fold(0, |carry,(_,&value)| {
                                                                             let mut inc = 0;
                                                                             if value == group_size {
                                                                                 inc = 1;
                                                                             }
                                                                             carry + inc
                                                                         });

                                   all_answered_count
                               })
                                // finally sum the result
                               .fold(0, |carry, x| carry + x);
        println!("Yes count: {}", yes_count);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
