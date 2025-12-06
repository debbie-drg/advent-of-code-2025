advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<i64> {
    let (numbers_text, operations) = input.trim().rsplit_once("\n").unwrap();
    let operations = operations.split_whitespace().collect::<Vec<&str>>();
    let parsed_numbers: Vec<Vec<i64>> = numbers_text
        .split("\n")
        .into_iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|entry| entry.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    Some(
        operations
            .into_iter()
            .enumerate()
            .map(|(index, operation)| match operation {
                "*" => {
                    let mut result: i64 = 1;
                    for number in &parsed_numbers {
                        result *= number[index];
                    }
                    result
                }
                "+" => {
                    let mut result: i64 = 0;
                    for number in &parsed_numbers {
                        result += number[index];
                    }
                    result
                }
                _ => panic!(),
            })
            .sum::<i64>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (numbers_text, operations) = input.strip_suffix("\n").unwrap().rsplit_once("\n").unwrap();
    let operations: Vec<char> = operations.chars().collect();
    let split_numbers: Vec<Vec<char>> = numbers_text
        .split("\n")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut operation_placements: Vec<usize> = operations
        .iter()
        .enumerate()
        .filter(|(_, char)| char != &&' ')
        .map(|(index, _)| index)
        .collect();
    operation_placements.push(operations.len() + 1);

    let mut sum: i64 = 0;
    for operation_index in 0..operation_placements.len() - 1 {
        let mut numbers: Vec<i64> = Vec::new();
        for index in
            operation_placements[operation_index]..operation_placements[operation_index + 1] - 1
        {
            let mut current_number: String = "".to_owned();
            for number in &split_numbers {
                current_number.push(number[index]);
            }
            numbers.push(current_number.trim().parse().unwrap());
        }
        match operations[operation_placements[operation_index]] {
            '*' => {
                sum += numbers.iter().copied().reduce(|a, b| a * b).unwrap();
            }
            '+' => {
                sum += numbers.iter().sum::<i64>();
            }
            _ => panic![],
        }
    }
    Some(sum)
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
