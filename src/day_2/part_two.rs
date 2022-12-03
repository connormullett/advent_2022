#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Option {
    Rock,
    Paper,
    Scissors
}

impl Option {
    pub fn score(&self) -> u32 {
        match self {
            Option::Rock => 1,
            Option::Paper => 2,
            Option::Scissors => 3,
        }
    }

    pub fn compare(&self, other: &Outcome) -> Option {
        match self {
            Option::Rock if other == &Outcome::Win => Option::Paper,
            Option::Rock if other == &Outcome::Loss => Option::Scissors,
            Option::Rock if other == &Outcome::Draw => Option::Rock,
            Option::Paper if other == &Outcome::Win => Option::Scissors,
            Option::Paper if other == &Outcome::Loss => Option::Rock,
            Option::Paper if other == &Outcome::Draw => Option::Paper,
            Option::Scissors if other == &Outcome::Win => Option::Rock,
            Option::Scissors if other == &Outcome::Loss => Option::Paper,
            Option::Scissors if other == &Outcome::Draw => Option::Scissors,
            _ => panic!("Invalid")
        }
    }
}

impl From<&str> for Option {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Option::Rock,
            "B" | "Y" => Option::Paper,
            "C" | "Z" => Option::Scissors,
            _ => panic!("Invalid input")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Loss,
    Draw
}

impl From<&str> for Outcome {
    fn from(c: &str) -> Self {
        match c {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid input")
        }
    }
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3
        }
    }
}

#[allow(unused)]
pub fn solve(input: String) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut score = 0;

    for line in lines {
        let mut line = line.split(' ');
        let opponent_move = line.next().unwrap();

        let opp_option = Option::from(opponent_move);
        let best_outcome = Outcome::from(line.next().unwrap());

        let option = opp_option.compare(&best_outcome);

        let result = option.score() + best_outcome.score();
        score += result;
    }

    println!("result :: {}", score);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::solve;

    #[test]
    fn part_two() {
        let input = read_to_string("inputs/day_2_input.txt").unwrap();
        solve(input);
    }
}

