use std::cmp::Reverse;
advent_of_code::solution!(3);

fn parse_string_to_numbers(input: &str) -> Vec<u64> {
    input
        .chars()
        .map(|char| {
            char.to_string()
                .parse::<u64>()
                .expect("Failed to parse integer")
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .split("\n")
        .map(parse_string_to_numbers)
        .collect()
}

fn compute_max_joltage(input: &[u64], length: usize) -> Option<u64> {
    let mut joltage: u64 = 0;
    let mut position = 0;
    for digit_counter in (0..length).rev() {
        let (max_position, max_value) = input[position..input.len() - digit_counter]
            .iter()
            .enumerate()
            .min_by_key(|&(_, value)| Reverse(value))?;

        position += max_position + 1;
        joltage += (10_u64).pow(digit_counter as u32) * max_value;
    }
    Some(joltage)
}

fn sum_joltages(input: &str, length: usize) -> Option<u64> {
    parse_input(input)
        .iter()
        .map(|line| compute_max_joltage(line, length))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    sum_joltages(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    sum_joltages(input, 12)
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
        assert_eq!(result, Some(3121910778619));
    }
}
