use std::fs;
use std::io::Read;
use std::str::FromStr;

pub fn day_2() {
    let mut file = fs::File::open("./input_files/day2_input.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    println!(
        "Part 1: total score: {}.",
        play_rock_paper_scissors_part1(&data)
    );
    println!(
        "Part 2: total score: {}.",
        play_rock_paper_scissors_part2(&data)
    );
}

fn play_rock_paper_scissors_part1(input: &String) -> u32 {
    let mut total_score = 0;
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            let (opponent, player) = line.split_once(' ').unwrap();
            let opponent = HandState::from_str(opponent).unwrap();
            let player = HandState::from_str(player).unwrap();
            total_score += get_outcome_of_round(opponent, player) as u32;
            total_score += player as u32;
        }
    }
    total_score
}

fn play_rock_paper_scissors_part2(input: &String) -> u32 {
    let mut total_score = 0;
    for mut line in input.lines() {
        line = line.trim();
        if !line.is_empty() {
            let (opponent, outcome) = line.split_once(' ').unwrap();
            let opponent = HandState::from_str(opponent).unwrap();
            let outcome = Outcome::from_str(outcome).unwrap();
            let player = get_hand_state_by_expected_outcome(opponent, outcome);
            total_score += get_outcome_of_round(opponent, player) as u32;
            total_score += player as u32;
        }
    }
    total_score
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum HandState {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for HandState {
    type Err = ();
    fn from_str(input: &str) -> Result<HandState, Self::Err> {
        match input {
            "A" => Ok(HandState::Rock),
            "X" => Ok(HandState::Rock),
            "B" => Ok(HandState::Paper),
            "Y" => Ok(HandState::Paper),
            "C" => Ok(HandState::Scissors),
            "Z" => Ok(HandState::Scissors),
            _ => Err(()),
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn get_hand_state_by_expected_outcome(opponent: HandState, expected_outcome: Outcome) -> HandState {
    match (opponent, expected_outcome) {
        (HandState::Rock, Outcome::Lose) | (HandState::Paper, Outcome::Win) => HandState::Scissors,
        (HandState::Paper, Outcome::Lose) | (HandState::Scissors, Outcome::Win) => HandState::Rock,
        (HandState::Scissors, Outcome::Lose) | (HandState::Rock, Outcome::Win) => HandState::Paper,
        (a, Outcome::Draw) => a,
    }
}

fn get_outcome_of_round(opponent: HandState, player: HandState) -> Outcome {
    match (opponent, player) {
        (a, b) if a == b => Outcome::Draw,
        (HandState::Scissors, HandState::Rock)
        | (HandState::Rock, HandState::Paper)
        | (HandState::Paper, HandState::Scissors) => Outcome::Win,
        (_, _) => Outcome::Lose,
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::{play_rock_paper_scissors_part1, play_rock_paper_scissors_part2};

    #[test]
    fn test_play_rock_paper_scissors() {
        let input = String::from(
            "A Y
            B X
            C Z
            B Z",
        );
        assert_eq!(play_rock_paper_scissors_part1(&input), 24);
    }

    #[test]
    fn test_play_rock_paper_scissors_part2() {
        let input = String::from(
            "A Y
            B X
            C Z",
        );
        assert_eq!(play_rock_paper_scissors_part2(&input), 12);
    }
}
