
use std::fs::read_to_string;

mod day_1;

fn main() {
    let day_1_input = read_to_string("inputs/day_1_input.txt").unwrap();
    day_1::solve(day_1_input);
}
