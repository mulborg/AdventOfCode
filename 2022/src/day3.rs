use std::fs;
use std::io::Read;

pub fn day_3() {
    let mut file = fs::File::open("./input_files/day3_input.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    println!(
        "Part 1: sum of the priorities: {}.",
        calculate_sum_of_priorities(&data)
    );
    println!(
        "Part 2: sum of group priorities: {}.",
        calculate_sum_of_priorities_for_groups(&data)
    );
}

fn calculate_sum_of_priorities(input: &String) -> u32 {
    let mut sum_of_priorities = 0;
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            sum_of_priorities += calculate_priority_of_rucksack(line);
        }
    }
    sum_of_priorities
}

fn calculate_sum_of_priorities_for_groups(input: &String) -> u32 {
    let mut sum_of_priorities = 0;
    let mut lines = Vec::new();
    for mut line in input.lines() {
        line = line.trim();
        lines.push(line);
        if lines.len() == 3 {
            sum_of_priorities += calculate_priority_of_group(&lines);
            lines = Vec::new();
        }
    }
    sum_of_priorities
}

fn calculate_priority_of_rucksack(input: &str) -> u32 {
    let mut priority = 0;
    if (input.len() % 2) != 0 {
        panic!("Input length is odd!")
    }
    let half_length = input.len() / 2;
    priority += match contains_at_least_one_char_in_every_input(
        &vec![&input[half_length..]],
        &input[0..half_length],
    ) {
        Some(item_type) => calculate_priority(item_type),
        None => 0,
    };
    priority
}

fn calculate_priority_of_group(input: &[&str]) -> u32 {
    let mut priority = 0;
    priority += match contains_at_least_one_char_in_every_input(&input[1..], &input[0]) {
        Some(item_type) => calculate_priority(item_type),
        None => 0,
    };
    priority
}

fn contains_at_least_one_char_in_every_input(input: &[&str], chars: &str) -> Option<char> {
    for c in chars.chars() {
        let mut counter = 0;
        for input_item in input {
            if input_item.contains(c) {
                counter += 1
            }
        }
        if counter == input.len() {
            return Some(c);
        }
    }
    None
}

fn calculate_priority(item_type: char) -> u32 {
    if !item_type.is_ascii_alphabetic() {
        panic!("Invalid character!")
    }
    if item_type.is_ascii_lowercase() {
        item_type as u32 - 96
    } else {
        item_type as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{calculate_sum_of_priorities, calculate_sum_of_priorities_for_groups};

    #[test]
    fn test_calculate_sum_of_priorities() {
        let input = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(calculate_sum_of_priorities(&input), 157);
    }

    #[test]
    fn test_calculate_sum_of_priorities_for_groups() {
        let input = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(calculate_sum_of_priorities_for_groups(&input), 70);
    }
}
