advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

fn parse_manifold(input: &str) -> (HashSet<(u64, u64)>, (u64, u64)) {
    let mut splitters = HashSet::new();
    let mut start: (u64, u64) = (0, 0);
    let split_input: Vec<&str> = input.trim().split("\n").into_iter().collect();
    for (row, line) in split_input.iter().rev().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '^' {
                splitters.insert((row as u64, col as u64));
            } else if ch == 'S' {
                start = (row as u64, col as u64);
            }
        }
    }
    (splitters, start)
}

fn beam_count(splitters: &HashSet<(u64, u64)>, start_position: (u64, u64)) -> (u64, u64) {
    let mut used_splitters: HashSet<(u64, u64)> = HashSet::new();
    let mut current_beams: HashMap<u64, u64> = HashMap::new();
    current_beams.insert(start_position.1, 1);
    let mut next_beams: HashMap<u64, u64> = HashMap::new();
    for row in (0..start_position.0).rev() {
        for (col, beam_count) in current_beams.iter().into_iter() {
            if splitters.contains(&(row, *col)) {
                next_beams
                    .entry(*col - 1)
                    .and_modify(|count| *count += beam_count)
                    .or_insert(*beam_count);
                next_beams
                    .entry(*col + 1)
                    .and_modify(|count| *count += beam_count)
                    .or_insert(*beam_count);
                used_splitters.insert((row, *col));
            } else {
                next_beams
                    .entry(*col)
                    .and_modify(|count| *count += beam_count)
                    .or_insert(*beam_count);
            }
        }
        current_beams = next_beams;
        next_beams = HashMap::new();
    }
    let splitter_count = used_splitters.len() as u64;
    let beam_count = current_beams
        .iter()
        .into_iter()
        .map(|(_, value)| value)
        .sum();
    (splitter_count, beam_count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (splitters, start_position) = parse_manifold(input);
    let (splitter_count, _) = beam_count(&splitters, start_position);
    Some(splitter_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (splitters, start_position) = parse_manifold(input);
    let (_, beam_count) = beam_count(&splitters, start_position);
    Some(beam_count)
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
