advent_of_code::solution!(5);

use std::cmp::{max, min};

fn parse_fresh_intervals(input: &str) -> Option<(Vec<(u64, u64)>, Vec<u64>)> {
    let mut split_ingredients = input.trim().split("\n\n");
    let fresh_intervals = split_ingredients.next()?;
    let ingredients = split_ingredients.next()?;
    let ingredient_intervals: Vec<(u64, u64)> = fresh_intervals
        .split("\n")
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            Some((start, end))
        })
        .filter_map(|x| x)
        .collect();
    let ingredient_list = ingredients
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|ingredient| Some(ingredient.parse::<u64>().ok()?))
        .filter_map(|x| x)
        .collect();
    Some((ingredient_intervals, ingredient_list))
}

fn in_interval(ingredient: u64, fresh_interval: &(u64, u64)) -> bool {
    fresh_interval.0 <= ingredient && ingredient <= fresh_interval.1
}

fn intervals_intersect(interval_1: &(u64, u64), interval_2: &(u64, u64)) -> bool {
    in_interval(interval_1.0, interval_2)
        || in_interval(interval_1.1, interval_2)
        || in_interval(interval_2.1, interval_1)
}

fn interval_intersection(interval_1: &(u64, u64), interval_2: &(u64, u64)) -> (u64, u64) {
    (
        min(interval_1.0, interval_2.0),
        max(interval_1.1, interval_2.1),
    )
}

fn merge_intervals(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    intervals.sort();
    let mut merged_intervals: Vec<(u64, u64)> = Vec::new();
    let mut current_interval: (u64, u64) = intervals[0];
    for interval in intervals {
        if intervals_intersect(&interval, &current_interval) {
            current_interval = interval_intersection(&interval, &current_interval)
        } else {
            merged_intervals.push(current_interval);
            current_interval = interval;
        }
    }
    merged_intervals.push(current_interval);
    merged_intervals
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ingredient_intervals, ingredients) = parse_fresh_intervals(input)?;
    ingredient_intervals = merge_intervals(ingredient_intervals);
    Some(
        ingredients
            .into_iter()
            .map(|ingredient| {
                ingredient_intervals
                    .iter()
                    .any(|interval| in_interval(ingredient, &interval))
            })
            .filter(|fresh| *fresh)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut ingredient_intervals, _) = parse_fresh_intervals(input)?;
    ingredient_intervals = merge_intervals(ingredient_intervals);
    Some(
        ingredient_intervals
            .into_iter()
            .map(|interval| interval.1 - interval.0 + 1)
            .sum(),
    )
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
        assert_eq!(result, Some(14));
    }
}
