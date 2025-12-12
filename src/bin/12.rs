advent_of_code::solution!(12);

use std::iter::zip;

fn present_sizes(maps: &str) -> Vec<usize> {
    maps.split("\n\n")
        .map(|present| present.chars().filter(|char| *char == '#').count())
        .collect()
}

fn parse_region(instruction: &str) -> Option<(usize, Vec<usize>)> {
    let (grid, counts) = instruction.split_once(":")?;
    let (number_1, number_2) = grid.split_once("x")?;
    let number_1 = number_1.parse::<usize>().ok()?;
    let number_2 = number_2.parse::<usize>().ok()?;
    let counts = counts
        .split(" ")
        .filter_map(|count| count.parse().ok())
        .collect();
    Some((number_1 * number_2, counts))
}

fn is_region_valid(sizes: &[usize], region: &(usize, Vec<usize>)) -> bool {
    let (total_size, number_presents) = region;
    let total_count: usize = zip(sizes, number_presents)
        .map(|(size, number)| size * number)
        .sum();
    total_count <= *total_size
}

pub fn part_one(input: &str) -> Option<u64> {
    let (maps, instructions) = input.rsplit_once("\n\n")?;
    let sizes = present_sizes(maps);
    let regions: Vec<(usize, Vec<usize>)> =
        instructions.split("\n").filter_map(parse_region).collect();
    Some(
        regions
            .iter()
            .map(|region| is_region_valid(&sizes, region) as u64)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}
