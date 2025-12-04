advent_of_code::solution!(4);

use std::collections::HashSet;

static ADJACENCY: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn neighbours(position: (i64, i64)) -> Vec<(i64, i64)> {
    ADJACENCY
        .into_iter()
        .map(|adjacent| (position.0 + adjacent.0, position.1 + adjacent.1))
        .collect()
}

fn roll_positions(input: &str) -> HashSet<(i64, i64)> {
    let mut positions = HashSet::new();
    let split_input: Vec<&str> = input
        .split("\n")
        .into_iter()
        .filter(|line| line.len() > 0)
        .collect();
    for (row, line) in split_input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '@' {
                positions.insert((row as i64, col as i64));
            }
        }
    }
    positions
}

fn is_forklift_accessible(position: (i64, i64), rolls: &HashSet<(i64, i64)>) -> bool {
    neighbours(position)
        .iter()
        .filter(|neighbour| rolls.contains(neighbour))
        .count()
        < 4
}

fn forklift_accessible(input: &str, remove: bool) -> Option<u64> {
    let mut rolls_map = roll_positions(input);
    let mut accessible = 0;
    loop {
        let accessible_list: Vec<(i64, i64)> = rolls_map
            .iter()
            .filter(|position| is_forklift_accessible(**position, &rolls_map))
            .copied()
            .collect();
        let accessible_now = accessible_list.len();
        accessible += accessible_now as u64;
        if accessible_now == 0 || !remove {
            return Some(accessible as u64);
        }
        for position in accessible_list {
            rolls_map.remove(&position);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    forklift_accessible(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    forklift_accessible(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
