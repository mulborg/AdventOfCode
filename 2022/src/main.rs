use crate::day1::day_1;
use crate::day2::day_2;
use crate::day3::day_3;
use crate::day4::day_4;
use crate::day5::day_5;
use crate::day6::day_6;
use colored::Colorize;
use std::env;
use std::fs;
use std::io::Read;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("{}", "Advent of Code 2022".green());

    let args: Vec<String> = env::args().collect();
    let message = "Invalid day!";

    if args.len() < 2 {
        return show_error(message);
    };
    let day: u8 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => return show_error(message),
    };
    println!("{} {}:", "Day".yellow(), day.to_string().yellow());

    let mut file = match fs::File::open(format!("./input_files/day{}_input.txt", day)) {
        Ok(file) => file,
        Err(_) => return show_error(message),
    };
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Error while reading file");

    match day {
        1 => day_1(&input),
        2 => day_2(&input),
        3 => day_3(&input),
        4 => day_4(&input),
        5 => day_5(&input),
        6 => day_6(&input),
        _ => show_error(message),
    }
}

fn show_error(message: &str) {
    println!("{}", message.red());
}
