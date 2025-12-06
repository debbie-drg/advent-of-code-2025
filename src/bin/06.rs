use std::iter::zip;

advent_of_code::solution!(6);

fn operate(operand: char, numbers: Vec<u64>) -> u64 {
    match operand {
        '*' => numbers
            .iter()
            .copied()
            .reduce(|num_1, num_2| num_1 * num_2)
            .unwrap(),
        '+' => numbers.iter().sum(),
        _ => panic!("Impossible operand"),
    }
}

fn operate_all(operands: Vec<char>, numbers_list: Vec<Vec<u64>>) -> u64 {
    zip(operands, numbers_list)
        .into_iter()
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
    let (numbers_text, operands) = input.trim().rsplit_once("\n").unwrap();
    let operands = operands.chars().filter(|char| char != &' ').collect();
    let parsed_numbers: Vec<Vec<u64>> = numbers_text
        .split("\n")
        .into_iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|entry| entry.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let parsed_numbers = transpose(parsed_numbers);
    Some(operate_all(operands, parsed_numbers))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (numbers_text, operands) = input.strip_suffix("\n").unwrap().rsplit_once("\n").unwrap();
    let operand_chars: Vec<char> = operands.chars().collect();
    let split_numbers: Vec<Vec<char>> = numbers_text
        .split("\n")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut operation_placements: Vec<usize> = operand_chars
        .iter()
        .enumerate()
        .filter(|(_, char)| char != &&' ')
        .map(|(index, _)| index)
        .collect();
    operation_placements.push(operand_chars.len() + 1);

    let mut numbers_list: Vec<Vec<u64>> = Vec::new();

    for operation_index in 0..operation_placements.len() - 1 {
        let mut current_numbers: Vec<u64> = Vec::new();
        for index in
            operation_placements[operation_index]..operation_placements[operation_index + 1] - 1
        {
            let mut current_number: String = "".to_owned();
            for number in &split_numbers {
                current_number.push(number[index]);
            }
            current_numbers.push(current_number.trim().parse().unwrap());
        }
        numbers_list.push(current_numbers);
    }

    let operands: Vec<char> = operand_chars
        .into_iter()
        .filter(|char| char != &' ')
        .collect();
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
