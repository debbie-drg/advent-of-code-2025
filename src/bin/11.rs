advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse_node(input: &str) -> Option<(String, Vec<String>)> {
    let (name, children) = input.split_once(":")?;
    let children = children
        .split(" ")
        .filter(|name| !name.is_empty())
        .map(|name| name.to_owned())
        .collect();
    Some((name.to_owned(), children))
}

fn parse_nodes(input: &str) -> HashMap<String, Vec<String>> {
    input.split("\n").filter_map(parse_node).collect()
}

fn count_paths(
    boxes: &HashMap<String, Vec<String>>,
    start_position: &str,
    end_position: &str,
) -> u64 {
    let mut counts = HashMap::new();
    count_paths_recursor(boxes, start_position.to_owned(), end_position, &mut counts)
}

fn count_paths_recursor(
    boxes: &HashMap<String, Vec<String>>,
    current_position: String,
    end_position: &str,
    counts: &mut HashMap<String, u64>,
) -> u64 {
    let mut result: u64 = 0;
    for child in boxes[&current_position].clone() {
        if counts.contains_key(&child) {
            result += counts[&child];
        } else if child == end_position {
            result += 1;
        } else if boxes.contains_key(&child) {
            result += count_paths_recursor(boxes, child, end_position, counts)
        }
    }
    counts.insert(current_position, result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_nodes(input);
    Some(count_paths(&boxes, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_nodes(input);
    let fft_dac = count_paths(&boxes, "fft", "dac");

    let dac_fft = count_paths(&boxes, "dac", "fft");
    if fft_dac != 0 {
        let svr_fft = count_paths(&boxes, "svr", "fft");
        let dac_out = count_paths(&boxes, "dac", "out");

        return Some(svr_fft * fft_dac * dac_out);
    }
    let svr_dac = count_paths(&boxes, "svr", "dac");
    let fft_out = count_paths(&boxes, "fft", "out");
    Some(svr_dac * dac_fft * fft_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
