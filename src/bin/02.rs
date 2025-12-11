advent_of_code::solution!(2);

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter_map(|range| {
            let mut parts = range.split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect()
}

fn is_valid(id: &str, all_lengths: bool) -> bool {
    let as_bytes = id.as_bytes();
    let max_len = as_bytes.len() / 2;
    let chunk_sizes = if all_lengths {
        1..=max_len
    } else {
        if !as_bytes.len().is_multiple_of(2) {
            return false;
        };
        max_len..=max_len
    };
    for chunk_size in chunk_sizes {
        if as_bytes.len().is_multiple_of(chunk_size) {
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

fn count_all_valid(input: &str, all_lengths: bool) -> Option<u64> {
    let parsed_ranges = parse_ranges(input);
    Some(
        parsed_ranges
            .into_iter()
            .map(|range| count_valid(range, all_lengths))
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    count_all_valid(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    count_all_valid(input, true)
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
