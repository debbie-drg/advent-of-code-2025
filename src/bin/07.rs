advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet};

fn parse_manifold(input: &str) -> (HashSet<(i64, i64)>, (i64, i64)) {
    let mut splitters = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    let split_input: Vec<&str> = input.trim().split("\n").collect();
    for (row, line) in split_input
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, line)| line)
        .rev()
        .enumerate()
    {
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

fn beam_count(splitters: &HashSet<(i64, i64)>, start_position: (i64, i64)) -> (i64, i64) {
    let mut used_splitters: HashSet<(i64, i64)> = HashSet::new();
    let mut current_beams: HashMap<i64, i64> = HashMap::new();
    current_beams.insert(start_position.1, 1);
    let mut next_beams: HashMap<i64, i64> = HashMap::new();
    for row in (0..start_position.0).rev() {
        for (col, beam_count) in current_beams.iter() {
            let offsets: Vec<i64>;
            if splitters.contains(&(row, *col)) {
                offsets = vec![-1, 1];
                used_splitters.insert((row, *col));
            } else {
                offsets = vec![0];
            }
            for offset in offsets {
                next_beams
                    .entry(*col + offset)
                    .and_modify(|count| *count += beam_count)
                    .or_insert(*beam_count);
            }
        }
        current_beams = next_beams;
        next_beams = HashMap::new();
    }
    let splitter_count = used_splitters.len() as i64;
    let beam_count = current_beams.values().sum();
    (splitter_count, beam_count)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (splitters, start_position) = parse_manifold(input);
    let (splitter_count, _) = beam_count(&splitters, start_position);
    Some(splitter_count)
}

pub fn part_two(input: &str) -> Option<i64> {
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
