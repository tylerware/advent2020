use std::str::FromStr;
use std::string::ParseError;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Forward,
    Left,
    Right,

    East,
    West,
    North,
    South,

    Noop,
}

#[derive(Debug)]
pub struct Direction {
    pub command: Command,
    pub value: usize
}

impl FromStr for Direction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let regex: Regex = Regex::new(r"^(?P<command>[A-Z])(?P<value>[\d]+)$").unwrap();
        let mut value = 0;
        let mut command = Noop;
        for capture in regex.captures_iter(s) {
            value = usize::from_str(&capture["value"]).unwrap();
            command = match &capture["command"] {
                "N" => North,
                "S" => South,
                "E" => East,
                "W" => West,
                "L" => Left,
                "R" => Right,
                "F" => Forward,
                _ => Noop
            }
        }

        Ok(Direction {
            command: command,
            value: value
        })
    }
}

pub enum NavigationSystem {
    Standard,
    WayPoint
}

pub struct Ship {
    starting_position: (i32, i32),
    waypoint: (i32, i32),
    navigation_system: NavigationSystem,
    pub current_position: (i32, i32),
    pub angle: f64,
}

impl Ship {
    pub fn new(navigation_system: NavigationSystem) -> Self {
        Self {
            starting_position: (0,0),
            current_position: (0,0),
            waypoint: (10,1),
            angle: 0.0,
            navigation_system: navigation_system
        }
    }


    pub fn process(&mut self, direction: Direction) {
        use Command::*;
        match &direction.command {
            North | South | East | West | Forward => self.move_it(direction),
            Left | Right => self.turn_it(direction),
            Noop => ()
        }
    }

    fn move_it(&mut self, direction: Direction) {
        use Command::*;
        let dist = direction.value as i32;
        let delta = match direction.command {
            North => (0, dist),
            South => (0, -dist),
            East => (dist, 0),
            West => (-dist , 0),
            Forward => {
                match self.navigation_system {
                    NavigationSystem::Standard => {
                        let x = (dist as f64) * self.angle.cos();
                        let y = (dist as f64) * self.angle.sin();
                        (x.round() as i32, y.round() as i32)
                    },
                    NavigationSystem::WayPoint => {
                        (dist * self.waypoint.0, dist * self.waypoint.1)
                    }
                }
            },
            _ => panic!("Wrong move type")
        };

        match self.navigation_system {
            NavigationSystem::Standard => self.move_ship(delta),
            NavigationSystem::WayPoint => {
                if direction.command == Forward {
                    self.move_ship(delta);
                }
                else {
                    self.waypoint = (
                        delta.0 + self.waypoint.0,
                        delta.1 + self.waypoint.1
                    );
                    println!("Waypoint: {:?}", self.waypoint);
                }
            }
        }
    }

    fn move_ship(&mut self, delta: (i32,i32)) {
        self.current_position = (
            delta.0 + self.current_position.0,
            delta.1 + self.current_position.1
        );
    }


    fn turn_it(&mut self, direction: Direction) {
        use Command::*;
        let mut theta = (direction.value as f64).to_radians();
        theta = match direction.command {
            Left => theta,
            Right => -theta,
            _ => panic!("Wrong turn type")
        };


        match self.navigation_system {
            NavigationSystem::Standard => {
                self.angle += theta;
            },
            NavigationSystem::WayPoint => {
                /* Using:
                 *  | cos(theta), -sin(theta) | | x |
                 *  | sin(theta), cos(theta)  | | y |
                 * We get:
                 */
                let x = self.waypoint.0 as f64;
                let y = self.waypoint.1 as f64;
                self.waypoint = (
                    (theta.cos() * x - theta.sin() * y).round() as i32,
                    (theta.sin() * x + theta.cos() * y).round() as i32
                );
                println!("Waypoint: {:?}", self.waypoint);
            }
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        let diff = (
            self.starting_position.0 - self.current_position.0,
            self.starting_position.1 - self.current_position.1
        );

        (diff.0.abs() + diff.1.abs()) as usize
    }
}
