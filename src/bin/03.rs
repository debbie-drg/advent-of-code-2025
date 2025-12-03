advent_of_code::solution!(3);

fn parse_string_to_numbers(input: &str) -> Vec<u64> {
    input
        .chars()
        .into_iter()
        .map(|char| char.to_string().parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .split("\n")
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| parse_string_to_numbers(line))
        .collect()
}

fn compute_max_joltage(input: &Vec<u64>) -> u64 {
    let (position, value_1) = input[..input.len() - 1]
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|&(_, value)| value)
        .unwrap();
    let value_2 = input[input.len() - position - 1..].iter().max().unwrap();
    value_1 * 10 + value_2
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .iter()
            .map(|line| compute_max_joltage(line))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
