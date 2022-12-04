#![allow(dead_code)]

mod part_two;

#[derive(Debug, PartialEq)]
enum Option {
    Rock,
    Paper,
    Scissors,
}

impl Option {
    pub fn score(&self) -> u32 {
        match self {
            Option::Rock => 1,
            Option::Paper => 2,
            Option::Scissors => 3,
        }
    }

    pub fn compare(&self, other: Option) -> Outcome {
        match self {
            Option::Rock if other == Option::Rock => Outcome::Draw,
            Option::Rock if other == Option::Scissors => Outcome::Win,
            Option::Rock if other == Option::Paper => Outcome::Loss,
            Option::Paper if other == Option::Rock => Outcome::Win,
            Option::Paper if other == Option::Paper => Outcome::Draw,
            Option::Paper if other == Option::Scissors => Outcome::Loss,
            Option::Scissors if other == Option::Rock => Outcome::Loss,
            Option::Scissors if other == Option::Paper => Outcome::Win,
            Option::Scissors if other == Option::Scissors => Outcome::Draw,
            _ => panic!("Invalid"),
        }
    }
}

impl From<&str> for Option {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Option::Rock,
            "B" | "Y" => Option::Paper,
            "C" | "Z" => Option::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
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
        let my_move = line.next().unwrap();
        let opp_option = Option::from(opponent_move);
        let my_option = Option::from(my_move);

        let outcome = my_option.compare(opp_option);

        score += outcome.score();
        score += my_option.score();
    }

    println!("result :: {}", score);
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs::read_to_string;

    #[test]
    fn day_two() {
        let input = read_to_string("inputs/day_2_input.txt").unwrap();
        solve(input);
    }
}
