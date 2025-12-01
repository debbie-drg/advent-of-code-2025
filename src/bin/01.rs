advent_of_code::solution!(1);

fn parse_instruction(s: &str) -> Option<(char, i64)> {
    let mut chars = s.chars();
    let dir = chars.next()?;
    let num: i64 = chars.as_str().parse().ok()?; 

    Some((dir, num))
}

fn do_rotations(input: &str, count_all: bool) -> Option<i64> {
    let input_lines = input.split("\n");
    let mut value: i64 = 50;
    let mut next_value: i64;
    let mut count_zeros: i64 = 0;
    let instructions = input_lines
        .into_iter()
        .map(|instruction| parse_instruction(instruction).unwrap())
        .collect::<Vec<(char, i64)>>();
    for (instruction, move_number) in instructions {
        match instruction {
            'R' => next_value = value + move_number,
            _ => next_value = value - move_number,
        }
        let remainder = next_value.rem_euclid(100);
        if count_all {
            count_zeros += next_value.abs().div_euclid(100);
            if value != 0 && next_value <= 0 {
                count_zeros += 1
            }
        }
        else {
            if remainder == 0 {count_zeros += 1}
        }
        value = remainder;
    }
    Some(count_zeros)
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
