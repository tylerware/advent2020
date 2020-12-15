use crate::helpers;
use crate::input::Input;
use crate::solutions::day11::layout::{SeatLayout,PositionState,AdjacencyStrategy};

pub struct Day11;

// See https://adventofcode.com/2020/day/11
impl Day11 {

    fn get_seat_layout(filename: &String) -> SeatLayout {
        let wrapped_input = helpers::read_all::<String>(&filename);
        let seating_rows = wrapped_input.into_iter()
                                      .map(|s| {
                                          s.unwrap()
                                           .chars()
                                           .map(|x| match x {
                                               'L' => PositionState::Empty,
                                               '.' | _ => PositionState::NoChair,
                                           })
                                           .collect()
                                      })
                        .collect::<Vec<_>>();
       SeatLayout::new(seating_rows)
    }


    fn solution1(filename: &String) {
        let mut seat_layout = Self::get_seat_layout(filename);
        seat_layout.set_adjacencies(AdjacencyStrategy::CloseBy);
        loop {
            let new_layout = seat_layout.apply_seating_rules();
            if new_layout.changes == 0 {
                break;
            }

            seat_layout = new_layout;
        }
        println!("Final layout: \n{}", seat_layout);
        println!("Seats filled: {}", seat_layout.seats_filled());
    }

    fn solution2(filename: &String) {
        let mut seat_layout = Self::get_seat_layout(filename);
        seat_layout.set_adjacencies(AdjacencyStrategy::LineOfSight);
        seat_layout.moving_tolerance = 5;
        loop {
            let new_layout = seat_layout.apply_seating_rules();
            if new_layout.changes == 0 {
                break;
            }

            seat_layout = new_layout;
        }
        println!("Final layout: \n{}", seat_layout);
        println!("Seats filled: {}", seat_layout.seats_filled());
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
