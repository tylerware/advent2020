use crate::helpers;
use crate::input::Input;
use crate::solutions::day5::seating::{Seat};

pub struct Day5;

// See https://adventofcode.com/2020/day/5
impl Day5 {
    fn get_seats(filename: &String) -> Vec<Seat> {
        let wrapped_input = helpers::read_all::<Seat>(&filename);
        wrapped_input.into_iter()
                     .map(|n| n.unwrap())
                     .collect::<Vec<_>>()
    }

    fn solution1(filename: &String) {
        let seats = Self::get_seats(filename);
        let max_seat_id = seats.iter()
                               .map(|x| x.seat_id())
                               .max()
                               .unwrap();
        println!("Max seat id: {}", max_seat_id);
    }

    fn solution2(filename: &String) {
        let mut seat_ids = Self::get_seats(filename).iter()
                                                    .map(|seat| seat.seat_id())
                                                    .collect::<Vec<usize>>();
        seat_ids.sort();
        let seat_offset = seat_ids.get(0).unwrap();
        let (my_seat, _) = seat_ids.iter()
                .enumerate()
                .map(|(i,&x)| (i + seat_offset, x))
                .find(|(i,x)| i != x)
                .unwrap();
        println!("My seat is {:?}", my_seat);
    }

    pub fn solution(input: &Input) {
        match &input.challenge {
            1 => Self::solution1(&input.filename),
            2 => Self::solution2(&input.filename),
            _ => println!("No solution {}.", &input.challenge)
        }
    }
}
