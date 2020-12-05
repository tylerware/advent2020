use std::str::FromStr;
use std::string::ParseError;

pub struct Seat {
    pub row: usize,
    pub column: usize
}

impl Seat {
    pub fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = ParseError;

    // the seat id is just a binary number where
    // - F and L map to 0
    // - B and R map to 1
    // The 'column' is the first 3 digits of the number
    // The 'row' is the 4th through 10th digit of the number
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Make it 'binary'
        let line: Vec<usize > = s.chars()
                                .map(|x| {
                                    match x {
                                       'F' | 'L' => 0,
                                       _ => 1
                                    }
                                })
                                .collect();


        let base: usize = 2;
        let row = line[0..7].iter()
                            .rev()
                            .enumerate()
                            .fold(0, |carry, (i,x)| carry + *x * base.pow(i as u32));
        let column = line[7..].iter()
                              .rev()
                              .enumerate()
                              .fold(0, |carry, (i,x)| carry + *x * base.pow(i as u32));


        let seat = Seat {
            row: row,
            column: column
        };
        Ok(seat)
    }
}
