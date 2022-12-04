#![allow(unused)]

pub fn solve(input: String) {
    let lines: Vec<&str> = input.lines().collect();

    let sum = lines
        .chunks(3)
        .map(|chunk| {
            for c in chunk[0].chars() {
                if chunk[1].contains(c) && chunk[2].contains(c) {
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
    use std::fs::read_to_string;
    use super::solve;

    #[test]
    fn day_three_part_two() {
        let input = read_to_string("inputs/day_3_input.txt").unwrap();
        solve(input);
    }
}
