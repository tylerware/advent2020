use crate::helpers;
use crate::input::Input;
use crate::solutions::day7::bag::{BagRule};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

pub struct Day7;

// See https://adventofcode.com/2020/day/7
impl Day7 {
    // Perhaps unnecessary work done here, but I wanted to use VecDeque :)
    // Computes all the bags contained in each bag (strong assumption that there are no circular bags!!)
    fn get_bag_contents(filename: &String) -> HashMap<String, HashMap<String,usize>> {
        let bag_rules = helpers::read_all::<BagRule>(&filename)
            .into_iter()
            .map(|rule| rule.unwrap());

        let mut resolved_bags: HashMap<String, HashMap<String,usize>> = HashMap::new();
        let mut unresolved_bags = VecDeque::from_iter(bag_rules);
        while !unresolved_bags.is_empty() {
            let unresolved_bag = unresolved_bags.pop_front().unwrap();
            let are_contained_bags_resolved = &unresolved_bag.contained_bags.keys()
                                                                        .all(|key| resolved_bags.contains_key(key));
            // Are the bag's "dependencies" resolved?
            // If so, resolve the bag
            if *are_contained_bags_resolved {
                let mut resolved_bag: HashMap<String, usize> = HashMap::new();
                for (color, &count) in unresolved_bag.contained_bags.iter() {
                    let contained_bag = resolved_bags.get(color).unwrap();

                    for (inner_color, inner_count) in contained_bag.iter() {
                        let mut qty = count * inner_count;
                        if resolved_bag.contains_key(&inner_color.to_string()) {
                            let current_qty = resolved_bag.get(&inner_color.to_string()).unwrap();
                            qty += current_qty;
                        }

                        resolved_bag.insert(inner_color.to_string(), qty);
                    }


                    let mut qty = count;
                    if resolved_bag.contains_key(&color.to_string()) {
                        let current_qty = resolved_bag.get(&color.to_string()).unwrap();
                        qty += current_qty;
                    }
                    resolved_bag.insert(color.to_string(), qty);
                }


                resolved_bags.insert(unresolved_bag.color, resolved_bag);
            }
            else {
                unresolved_bags.push_back(unresolved_bag);
            }

        }
        resolved_bags
    }


    fn solution1(filename: &String) {
        let bag_contents = Self::get_bag_contents(filename);
        let bag_color = "shiny gold".to_string();
        let bags_containing_color = bag_contents.iter()
                                                .fold(0, |carry,(_color, content)| {
                                                    let mut inc = 0;
                                                    if content.contains_key(&bag_color) {
                                                        inc = 1;
                                                    }

                                                    carry + inc
                                                });
        println!("There are {} bags containing '{}'", bags_containing_color, bag_color);
    }

    fn solution2(filename: &String) {
        let bag_contents = Self::get_bag_contents(filename);
        let bag_color = "shiny gold".to_string();
        let bags_in_bag = bag_contents.get(&bag_color);

        println!("{:?}", &bags_in_bag);
        let bags_in_bag_count = bags_in_bag
                                      .unwrap()
                                      .values()
                                      .fold(0, |carry,count| carry + count);

        println!("There are {} bags in '{}'", bags_in_bag_count, bag_color);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
