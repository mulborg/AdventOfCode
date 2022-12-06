use regex::Regex;

pub fn day_5(input: &str) {
    println!(
        "Part 1: top of stacks after the rearrangement with CrateMover 9000: {}",
        rearrange_stacks_with_cratemover9000(input)
    );
    println!(
        "Part 2: top of stacks after the rearrangement with CrateMover 9001: {}",
        rearrange_stacks_with_cratemover9001(input)
    );
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn rearrange_stacks_with_cratemover9000(input: &str) -> String {
    let (mut stacks, moves) = get_stacks_and_moves(input);
    perform_moves_with_cratemover9000(&mut stacks, moves);
    get_top_of_stacks(&stacks)
}

fn rearrange_stacks_with_cratemover9001(input: &str) -> String {
    let (mut stacks, moves) = get_stacks_and_moves(input);
    perform_moves_with_cratemover9001(&mut stacks, moves);
    get_top_of_stacks(&stacks)
}

fn get_stacks_and_moves(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut stack_lines: Vec<&str> = Vec::new();
    let mut count_of_stacks = 0;
    let mut moves: Vec<Move> = Vec::new();
    let re = Regex::new(r".*(\d)").unwrap();
    for line in input.lines() {
        if !line.is_empty() {
            let first_char = line.trim().chars().next().unwrap();
            match first_char {
                '[' => stack_lines.push(line),
                c if c.is_ascii_digit() => {
                    count_of_stacks = re
                        .captures(line)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap()
                }
                _ => moves.push(parse_move(line)),
            }
        }
    }
    stack_lines.reverse();
    let stacks = parse_stacks(count_of_stacks, stack_lines);
    (stacks, moves)
}

fn parse_move(input: &str) -> Move {
    let parts: Vec<&str> = input.split(' ').collect();
    Move {
        count: parts[1].parse().unwrap(),
        from: parts[3].parse().unwrap(),
        to: parts[5].parse().unwrap(),
    }
}

fn parse_stacks(count_of_stacks: usize, stack_lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..count_of_stacks {
        stacks.push(vec![]);
    }
    for stack_line in stack_lines {
        for (i, item) in stacks.iter_mut().enumerate() {
            if let Some(c) = stack_line.chars().nth(i * 4 + 1) {
                if c.is_ascii_alphabetic() {
                    item.push(c);
                }
            }
        }
    }
    stacks
}

fn perform_moves_with_cratemover9000(stacks: &mut [Vec<char>], moves: Vec<Move>) {
    for m in moves {
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(c);
        }
    }
}

fn perform_moves_with_cratemover9001(stacks: &mut [Vec<char>], moves: Vec<Move>) {
    for m in moves {
        let start = stacks[m.from - 1].len() - m.count;
        let mut crates = stacks[m.from - 1].split_off(start);
        stacks[m.to - 1].append(&mut crates);
    }
}

fn get_top_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut top_of_stacks = String::new();
    for stack in stacks {
        top_of_stacks.push(stack.last().copied().unwrap());
    }
    top_of_stacks
}

#[cfg(test)]
mod tests {
    use crate::day5::{rearrange_stacks_with_cratemover9000, rearrange_stacks_with_cratemover9001};

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_rearrange_stacks_with_cratemover9000() {
        assert_eq!(
            rearrange_stacks_with_cratemover9000(&INPUT.to_string()),
            "CMZ"
        );
    }

    #[test]
    fn test_rearrange_stacks_with_cratemover9001() {
        assert_eq!(
            rearrange_stacks_with_cratemover9001(&INPUT.to_string()),
            "MCD"
        );
    }
}
