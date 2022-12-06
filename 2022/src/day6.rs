pub fn day_6(input: &str) {
    println!(
        "Part 1: number of characters processed before the first start-of-packet marker is detected: {}",
        get_index_of_char_after_first_x_distinct_chars(input, 4)
    );
    println!(
        "Part 2: number of characters processed before the first start-of-message marker is detected: {}",
        get_index_of_char_after_first_x_distinct_chars(input, 14)
    );
}

fn get_index_of_char_after_first_x_distinct_chars(
    input: &str,
    number_of_distinct_chars: usize,
) -> usize {
    let index = input
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(number_of_distinct_chars)
        .find(|w| {
            for i in 0..number_of_distinct_chars - 1 {
                for j in i + 1..number_of_distinct_chars {
                    if w[i].1 == w[j].1 {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()
        .last()
        .unwrap()
        .0;
    index + 1
}

#[cfg(test)]
mod tests {
    use crate::day6::get_index_of_char_after_first_x_distinct_chars;
    fn get_test_input() -> Vec<String> {
        vec![
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ]
    }

    #[test]
    fn test_get_index_of_char_after_first_x_distinct_chars() {
        for i in get_test_input().iter().zip(vec![7, 5, 6, 10, 11].iter()) {
            assert_eq!(get_index_of_char_after_first_x_distinct_chars(i.0, 4), *i.1);
        }
        for i in get_test_input().iter().zip(vec![19, 23, 23, 29, 26].iter()) {
            assert_eq!(
                get_index_of_char_after_first_x_distinct_chars(i.0, 14),
                *i.1
            );
        }
    }
}
