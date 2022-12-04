#![allow(unused)]

fn solve(input: String) {

}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::solve;

    #[test]
    fn day_three() {
        let input = read_to_string("inputs/day_3_input.txt").unwrap();
        solve(input);
    }
}
