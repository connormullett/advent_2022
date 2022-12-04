#![allow(unused)]

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .split('\n')
        .filter(|sector| !sector.is_empty())
        .map(|sector| {
            sector
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|subsector| {
                    subsector
                        .split('-')
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn solve(input: String) {
    let sum = parse_input(input)
        .into_iter()
        .map(|sector_ranges| {
            u32::from(
                (sector_ranges[0] <= sector_ranges[2] && sector_ranges[1] >= sector_ranges[3])
                    || (sector_ranges[2] <= sector_ranges[0]
                        && sector_ranges[3] >= sector_ranges[1]),
            )
        })
        .sum::<u32>();
    println!("{sum}");
}

fn part_two(input: String) {
    let sum = parse_input(input).into_iter().map(|sector_range| {
        let first = sector_range[0]..=sector_range[1];
        let second = sector_range[2]..=sector_range[3];
        u32::from(
            second.contains(&sector_range[0])
                || second.contains(&sector_range[1])
                || first.contains(&sector_range[2])
                || first.contains(&sector_range[3]),
        )
    }).sum::<u32>();
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::{part_two, solve};
    use std::fs::read_to_string;

    #[test]
    fn day_four() {
        let input = read_to_string("inputs/day_4_input.txt").unwrap();
        solve(input);
    }

    #[test]
    fn day_four_part_two() {
        let input = read_to_string("inputs/day_4_input.txt").unwrap();
        part_two(input);
    }
}
