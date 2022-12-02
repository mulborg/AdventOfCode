use crate::day1::day_1;
use crate::day2::day_2;
use colored::Colorize;
use std::env;

mod day1;
mod day2;

fn main() {
    println!("{}", "Advent of Code 2022".green());

    let args: Vec<String> = env::args().collect();
    let message = "Invalid day!";

    if args.len() < 2 {
        show_error(message);
        return;
    };
    let day: u8 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            show_error(message);
            return;
        }
    };
    match day {
        1 => day_1(),
        2 => day_2(),
        _ => show_error(message),
    }
}

fn show_error(message: &str) {
    println!("{}", message.red());
}
