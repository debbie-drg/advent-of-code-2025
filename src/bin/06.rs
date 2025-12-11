use std::iter::zip;

advent_of_code::solution!(6);

fn split_numbers_operands(input: &str) -> Option<(&str, Vec<char>)> {
    let (numbers_text, operands) = input.trim().rsplit_once("\n")?;
    let operands = operands.chars().filter(|char| char != &' ').collect();
    Some((numbers_text, operands))
}

fn operate(operand: char, numbers: Vec<u64>) -> u64 {
    match operand {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => panic!("Impossible operand"),
    }
}

fn operate_all(operands: Vec<char>, numbers_list: Vec<Vec<u64>>) -> u64 {
    zip(operands, numbers_list)
        .map(|(operand, numbers)| operate(operand, numbers))
        .sum()
}

fn transpose<T>(twod_array: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    (0..twod_array[0].len())
        .map(|col| {
            (0..twod_array.len())
                .map(|row| twod_array[row][col])
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (numbers_text, operands) = split_numbers_operands(input)?;
    let parsed_numbers: Vec<Vec<u64>> = transpose(
        numbers_text
            .split("\n")
            .map(|line| {
                line
                    .split_whitespace()
                    .filter_map(|entry| entry.parse::<u64>().ok())
                    .collect()
            })
            .collect(),
    );
    Some(operate_all(operands, parsed_numbers))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers_text, operands) = split_numbers_operands(input)?;
    let split_numbers: Vec<Vec<char>> = transpose(
        numbers_text
            .split("\n")
            .map(|line| line.chars().collect())
            .collect(),
    );
    let mut numbers_list: Vec<Vec<u64>> = Vec::new();
    let mut current_numbers: Vec<u64> = Vec::new();

    for line in split_numbers {
        let mut number_string = String::new();
        for char in line.iter() {
            number_string.push(*char);
        }
        match number_string.trim().parse::<u64>() {
            Ok(number) => current_numbers.push(number),
            Err(_) => {
                numbers_list.push(current_numbers);
                current_numbers = Vec::new();
            }
        }
    }
    numbers_list.push(current_numbers);

    Some(operate_all(operands, numbers_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
