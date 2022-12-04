use std::collections::BinaryHeap;

pub fn solve(input: String) {
    let elves: Vec<String> = input.split("\n\n").map(String::from).collect();

    let mut heap = elves
        .iter()
        .map(|elf| {
            elf.split("\n")
                .filter(|calorie| !calorie.is_empty())
                .map(|calorie| calorie.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<BinaryHeap<u32>>();

    let mut top_three = vec![];
    for _ in 0..3 {
        if let Some(v) = heap.pop() {
            top_three.push(v);
        }
    }
    let sum = top_three.iter().sum::<u32>();
    println!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs::read_to_string;

    #[test]
    fn day_one() {
        let input = read_to_string("inputs/day_1_input.txt").unwrap();
        solve(input);
    }
}
