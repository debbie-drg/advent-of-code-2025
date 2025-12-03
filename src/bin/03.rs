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

fn compute_max_joltage(input: &Vec<u64>, length: usize) -> u64 {
    let mut joltage: u64 = 0;
    let mut position = 0;
    for digit_counter in (0..length).rev() {
        let remaining_digits: Vec<&u64> = input[position..].iter().collect();

        let mut best: (usize, u64) = (0, 0);
        for (i, &value) in remaining_digits[..remaining_digits.len() - digit_counter]
            .iter()
            .enumerate()
        {
            match best {
                (_, max_val) if *value > max_val => best = (i, *value),
                _ => {} // ignore equal values, keeps the first
            }
        }

        position += best.0 + 1;
        joltage += (10 as u64).pow(digit_counter as u32) * best.1;
    }
    joltage
}

fn sum_joltages(input: &str, length: usize) -> u64 {
    parse_input(input)
        .iter()
        .map(|line| compute_max_joltage(line, length))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(sum_joltages(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sum_joltages(input, 12))
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
