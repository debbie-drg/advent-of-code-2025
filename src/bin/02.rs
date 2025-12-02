use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|s| Some(s))
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect()
}

fn is_valid(id: &str, all_lengths: bool) -> bool {
    let as_bytes = id.as_bytes();
    let max_len = as_bytes.len().div_euclid(2);
    let chunk_sizes: RangeInclusive<usize>;
    if all_lengths {
        chunk_sizes = 1..=max_len;
    } else {
        if as_bytes.len().rem_euclid(2) != 0 {
            return false;
        };
        chunk_sizes = max_len..=max_len;
    }
    for chunk_size in chunk_sizes {
        if as_bytes.len() % chunk_size == 0 {
            let first = &as_bytes[..chunk_size];
            if as_bytes.chunks(chunk_size).all(|chunk| chunk == first) {
                return true;
            };
        }
    }
    false
}

fn count_valid(range: (u64, u64), all_lengths: bool) -> u64 {
    let mut count = 0;
    for value in range.0..=range.1 {
        if is_valid(&value.to_string(), all_lengths) {
            count += value;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_ranges = parse_ranges(input);
    Some(
        parsed_ranges
            .into_iter()
            .map(|range| count_valid(range, false))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_ranges = parse_ranges(input);
    Some(
        parsed_ranges
            .into_iter()
            .map(|range| count_valid(range, true))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
