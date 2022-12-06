pub fn day_4(input: &str) {
    println!(
        "Part 1: count of assignment pairs where one range fully contains the other: {}",
        count_fully_contained_assignment_pairs(input)
    );
    println!(
        "Part 2: count of assignment pairs where ranges overlap: {}",
        count_overlapped_assignment_pairs(input)
    );
}

fn count_fully_contained_assignment_pairs(input: &str) -> u16 {
    let mut counter = 0;
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            let pair: Vec<&str> = line.split(',').collect();
            if pair.len() == 2 {
                let section1 = parse_section(pair[0]);
                let section2 = parse_section(pair[1]);
                let sections_merged = section1 | section2;
                if sections_merged == section1 || sections_merged == section2 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn count_overlapped_assignment_pairs(input: &str) -> u16 {
    let mut counter = 0;
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            let pair: Vec<&str> = line.split(',').collect();
            if pair.len() == 2 {
                let section1 = parse_section(pair[0]);
                let section2 = parse_section(pair[1]);
                if section1 & section2 != 0 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn parse_section(input: &str) -> u128 {
    let section: Vec<&str> = input.split('-').collect();
    if section.len() != 2 {
        panic!("Invalid section!");
    }
    let start: u8 = section[0].parse().unwrap();
    let end: u8 = section[1].parse().unwrap();
    let mut id = 0;
    for i in start..=end {
        id |= 1 << i;
    }
    id
}

#[cfg(test)]
mod tests {
    use crate::day4::{
        count_fully_contained_assignment_pairs, count_overlapped_assignment_pairs, parse_section,
    };

    #[test]
    fn test_count_fully_contained_assignment_pairs() {
        let input = String::from(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8",
        );
        assert_eq!(count_fully_contained_assignment_pairs(&input), 2);
    }

    #[test]
    fn test_count_overlapped_assignment_pairs() {
        let input = String::from(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8",
        );
        assert_eq!(count_overlapped_assignment_pairs(&input), 4);
    }

    #[test]
    fn test_parse_section() {
        let mut input = String::from("2-4");
        assert_eq!(parse_section(&input), 0b0001_1100);
        input = String::from("6-6");
        assert_eq!(parse_section(&input), 0b0100_0000);
        input = String::from("7-6");
        assert_eq!(parse_section(&input), 0b0000_0000);
        input = String::from("0-127");
        assert_eq!(parse_section(&input), !0b0);
    }
}
