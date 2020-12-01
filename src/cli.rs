use crate::input::Input;
use clap::{App,Arg};

use std::str::FromStr;

pub fn parse_input() -> Input {
    let app = App::new("advent2020")
        .version("0.1.0")
        .about("Solutions to the code challenges presented in the advent of code.")
        .author("Tyler Ware")
        .arg(Arg::with_name("day")
              .long("day")
              .short("d")
              .value_name("day")
              .required(true)
              .help("Which advent day?"))
        .arg(Arg::with_name("challenge")
              .long("challenge")
              .short("c")
              .value_name("challenge")
              .help("The challenge number. There are two challenges per day, so either 1 or 2."))
        .arg(Arg::with_name("file")
             .required(true)
             .help("The file containing the input data for the advent challenge."));

    let matches = app.get_matches();
    Input {
        day: usize::from_str(matches.value_of("day").unwrap()).unwrap(),
        challenge: usize::from_str(matches.value_of("challenge").unwrap_or("")).unwrap_or(1),
        filename: matches.value_of("file").unwrap().to_string()
    }
}
