#![allow(unused)]

mod part_two;

pub fn solve(input: String) {
    let lines: Vec<&str> = input.lines().collect();

    let sum = lines
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            for c in first.chars() {
                if second.contains(c) {
                    return get_priority(c);
                }
            }
            panic!("no dupes");
        })
        .sum::<usize>();
    println!("{sum}");
}

pub fn get_priority(c: char) -> usize {
    let c = c as u8;

    if c as u8 > 96 {
        (c - 96) as usize
    } else {
        (c - 38) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs::read_to_string;

    #[test]
    fn day_three() {
        let input = read_to_string("inputs/day_3_input.txt").unwrap();
        solve(input);
    }
}
