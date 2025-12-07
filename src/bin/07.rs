advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

fn parse_manifold(input: &str) -> (HashSet<(i64, i64)>, (i64, i64)) {
    let mut splitters = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    let split_input: Vec<&str> = input.trim().split("\n").into_iter().collect();
    for (row, line) in split_input.iter().rev().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                splitters.insert((row as i64, col as i64));
            } else if ch == 'S' {
                start = (row as i64, col as i64);
            }
        }
    }
    (splitters, start)
}

fn next_beam_count(
    splitters: &HashSet<(i64, i64)>,
    position: (i64, i64),
    memory: &mut HashMap<(i64, i64), i64>,
    used_splitters: &mut HashSet<(i64, i64)>,
) -> i64 {
    if memory.contains_key(&position) {
        return memory[&position];
    }
    let mut result: i64 = 0;
    let mut split: bool = false;
    let (hor_pos, ver_pos) = position;
    for row in (0..hor_pos).rev() {
        if splitters.contains(&(row, ver_pos)) {
            used_splitters.insert((row, ver_pos));
            result += next_beam_count(splitters, (row, ver_pos - 1), memory, used_splitters);
            result += next_beam_count(splitters, (row, ver_pos + 1), memory, used_splitters);
            split = true;
            break;
        }
    }
    if !split {
        result = 1
    };
    memory.insert(position, result);
    return result;
}

pub fn part_one(input: &str) -> Option<i64> {
    let (splitters, start_position) = parse_manifold(input);
    let mut memory: HashMap<(i64, i64), i64> = HashMap::new();
    let mut used_splitters: HashSet<(i64, i64)> = HashSet::new();
    _ = next_beam_count(&splitters, start_position, &mut memory, &mut used_splitters);
    Some(used_splitters.len() as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (splitters, start_position) = parse_manifold(input);
    let mut memory: HashMap<(i64, i64), i64> = HashMap::new();
    let mut used_splitters: HashSet<(i64, i64)> = HashSet::new();
    Some(next_beam_count(
        &splitters,
        start_position,
        &mut memory,
        &mut used_splitters,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
