use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionState {
    Empty,
    Full,
    NoChair
}

#[derive(Clone, Debug)]
pub struct Seat {
    position: usize,
    row: usize,
    column: usize,
    state: PositionState,
    // Convention is [NW,N,NE,W,E,SW,S,SE]
    pub adjacent_seats: Vec<i32>
}

pub enum AdjacencyStrategy {
    LineOfSight,
    CloseBy,
}

#[derive(Clone)]
pub struct SeatLayout {
    width: usize,
    height: usize,
    pub seats: Vec<Seat>,
    pub changes: usize,
    pub moving_tolerance: usize,
}

impl SeatLayout {
    pub fn new(seating_rows: Vec<Vec<PositionState>>) -> Self {
        let width = seating_rows[0].len();
        let height = seating_rows.len();
        println!("New layout of size {}x{}", width, height);

        let seats = seating_rows.iter()
                                .enumerate()
                                .fold(vec![], |mut seats, (i, row)| {
                                    let mut new_seats: Vec<Seat> = row.iter()
                                                       .enumerate()
                                                       .map(|(j, state)| Seat {
                                                           position: i * width + j,
                                                           row: i,
                                                           column: j,
                                                           state: *state,
                                                           adjacent_seats: vec![-1; 8]
                                                       })
                                                       .collect();
                                    seats.append(&mut new_seats);
                                    seats
                                });

        Self {
            width: width,
            height: height,
            seats: seats,
            changes: 0,
            moving_tolerance: 4
        }
    }

    pub fn set_adjacencies(&mut self, strategy: AdjacencyStrategy) {
        let seat_present: Vec<bool> = self.seats.iter()
                                                .map(|seat| seat.state != PositionState::NoChair)
                                                .collect();

        let seat_positions: Vec<(usize, usize)> = self.seats.iter()
                                                .map(|seat| (seat.row, seat.column))
                                                .collect();

        let directions: Vec<(i32,i32)> = vec![
            (-1,-1), (0,-1), (1,-1),
            (-1,0),          (1,0),
            (-1,1),  (0,1),  (1,1)
        ];

        let search_depth = match strategy {
            AdjacencyStrategy::CloseBy => 1,
            AdjacencyStrategy::LineOfSight => 0, // 0 means don't stop...
        };

        for (seat_row, seat_col) in seat_positions.iter() {
            let current_pos = (seat_row * self.width + seat_col) as i32;

            // only need seat adjacnecies for seats with chairs
            if !seat_present[current_pos as usize] {
                continue;
            }

            'direction: for (i, (d_col,d_row)) in directions.iter().enumerate() {
                // abort early if seat already has a seat in this direction
                if self.seats[current_pos as usize].adjacent_seats[i] >= 0 {
                    continue 'direction;
                }

                let mut j: i32 = 1;
                'search: loop {
                    let col = (*seat_col as i32) + j*d_col;
                    let row = (*seat_row as i32) + j*d_row;

                    // eventually we'll run out spaces, so we run out of bounds
                    if (col < 0 || col >= (self.width as i32))
                        || (row < 0 || row >= (self.height as i32)) {
                            break 'search;
                    }

                    let pos = row * (self.width as i32) + col;
                    if seat_present[pos as usize] {
                        self.seats[current_pos as usize].adjacent_seats[i] = pos;
                        self.seats[pos as usize].adjacent_seats[8 - (i + 1)] = current_pos;
                        break 'search;
                    }

                    // Note: if search_depth is 0, then we don't fall in here since j starts at 1
                    if j == search_depth {
                        break 'search;
                    }

                    j += 1;

                }
            }
        }
    }

    pub fn apply_seating_rules(&self) -> SeatLayout {
        let adjaceny_counts: Vec<usize> = self.seats.iter()
                                          .map(|seat| seat.adjacent_seats
                                               .iter()
                                               .fold(0, |c,pos| {
                                                   let mut inc = 0;
                                                   if *pos >= 0
                                                       && self.seats[*pos as usize].state == PositionState::Full {
                                                       inc = 1;
                                                   }
                                                   c + inc
                                               }))
                                          .collect();

        // println!("Before rule {:?} with {} adj seats", self.seats[0], adjaceny_counts[0]);
        let mut changes = 0;
        let seats: Vec<Seat> = adjaceny_counts.iter()
                                   .enumerate()
                                   .map(|(i, &c)| {
                                       let seat = self.seats[i].clone();
                                       let mut state = seat.state;
                                       if seat.state != PositionState::NoChair {
                                           if seat.state == PositionState::Empty
                                               && c == 0 {
                                                   changes += 1;
                                                   state = PositionState::Full;
                                           }


                                           if seat.state == PositionState::Full
                                               && c >= self.moving_tolerance {
                                                   changes += 1;
                                                   state = PositionState::Empty;
                                           }
                                       }
                                       let mut new_seat = seat.clone();
                                       new_seat.state = state;
                                       new_seat
                                   })
                                   .collect();

        Self {
            seats: seats,
            width: self.width,
            height: self.height,
            changes: changes,
            moving_tolerance: self.moving_tolerance,
        }
    }

    pub fn seats_filled (&self) -> usize {
        self.seats.iter()
                  .fold(0, |c, seat| {
                      let mut inc = 0;
                      if seat.state == PositionState::Full {
                          inc = 1;
                      }
                      c + inc
                  })
    }
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for seat in self.seats.iter() {
            let display_character = match seat.state {
                PositionState::Full => '#',
                PositionState::Empty => 'L',
                PositionState::NoChair => '.'
            };

            output.push(display_character);

            if (seat.column + 1) == self.width {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}
