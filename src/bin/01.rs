advent_of_code::solution!(1);

fn parse_instruction(s: &str) -> Option<(char, i64)> {
    let mut chars = s.chars();
    let dir = chars.next()?;
    let num: i64 = chars.as_str().parse().ok()?;

    Some((dir, num))
}

fn parse_all_instructions(input: &str) -> Vec<(char, i64)> {
    let input_lines: std::str::Split<'_, &str> = input.trim().split("\n");
    input_lines
        .into_iter()
        .map(|instruction| parse_instruction(instruction))
        .into_iter()
        .filter_map(|x| x)
        .collect()
}

fn do_rotations(input: &str, count_all: bool) -> Option<i64> {
    let instructions = parse_all_instructions(input);
    let mut value: i64 = 50;
    let mut next_value: i64;
    let mut part_1: i64 = 0;
    let mut part_2: i64 = 0;
    for (instruction, move_number) in instructions {
        match instruction {
            'R' => next_value = value + move_number,
            _ => next_value = value - move_number,
        }
        let remainder = next_value.rem_euclid(100);
        part_2 += next_value.abs() / 100;
        if value != 0 && next_value <= 0 {
            part_2 += 1
        }
        if remainder == 0 {
            part_1 += 1
        }
        value = remainder;
    }
    match count_all {
        true => Some(part_2),
        false => Some(part_1),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    do_rotations(input, false)
}

pub fn part_two(input: &str) -> Option<i64> {
    do_rotations(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
