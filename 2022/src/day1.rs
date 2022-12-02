use std::fs;
use std::io::Read;

pub fn day_1() {
    let mut file = fs::File::open("./input_files/day1_input.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let calories_by_elves = get_calories_by_elves(&data);
    let max_index_and_value = get_max_index_and_value(&calories_by_elves);
    let total_calories_of_top3_elves = get_sum_of_top_values(calories_by_elves, 3);
    println!(
        "Day 1:\nElf no.{} has {} calories.\nTotal calories of top 3 elves: {}.",
        max_index_and_value.0 + 1,
        max_index_and_value.1,
        total_calories_of_top3_elves
    );
}

fn get_calories_by_elves(input: &String) -> Vec<u32> {
    let mut calories = vec![0];
    for mut line in input.lines() {
        line = line.trim();
        if line.is_empty() {
            calories.push(0);
        } else {
            let item_calories: u32 = match line.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if let Some(last) = calories.last_mut() {
                *last += item_calories;
            }
        }
    }
    calories
}

fn get_max_index_and_value(items: &Vec<u32>) -> (usize, u32) {
    let mut max = (0, items[0]);
    for (index, &x) in items.iter().enumerate() {
        if x > max.1 {
            max.0 = index;
            max.1 = x;
        }
    }
    max
}

fn get_sum_of_top_values(mut items: Vec<u32>, count_of_items_to_sum: usize) -> u32 {
    items.sort_by(|a, b| b.cmp(a));
    if items.len() < count_of_items_to_sum {
        panic!("'items' length is smaller than 'count_of_items_to_sum'");
    }
    items[..count_of_items_to_sum].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{get_calories_by_elves, get_max_index_and_value, get_sum_of_top_values};

    #[test]
    fn test_get_calories_by_elves() {
        let input = String::from(
            "1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000",
        );
        let expected: Vec<u32> = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(get_calories_by_elves(&input), expected);
    }

    #[test]
    fn test_get_max_index_and_value() {
        let input: Vec<u32> = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(get_max_index_and_value(&input), (3, 24000));
    }

    #[test]
    fn test_get_sum_of_top_values() {
        let input: Vec<u32> = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(get_sum_of_top_values(input, 3), 45000);
    }
}
