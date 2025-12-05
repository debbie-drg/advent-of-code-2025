advent_of_code::solution!(5);

use std::cmp::{max, min};

fn parse_fresh_intervals(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut split_ingredients = input.split("\n\n");
    let fresh_intervals = split_ingredients.next().unwrap();
    let ingredients = split_ingredients.next().unwrap();
    let ingredient_intervals: Vec<(u64, u64)> = fresh_intervals
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect();
    let ingredient_list = ingredients
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|ingredient| ingredient.parse::<u64>().unwrap())
        .collect();
    (ingredient_intervals, ingredient_list)
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
    let mut checked_until = 0;
    loop {
        let mut changed = false;
        for index in checked_until..intervals.len() - 1 {
            if intervals_intersect(&intervals[index], &intervals[index + 1]) {
                intervals[index] = interval_intersection(&intervals[index], &intervals[index + 1]);
                intervals.remove(index + 1);
                checked_until = index;
                changed = true;
                break;
            }
        }
        if !changed {return intervals;}
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut ingredient_intervals, ingredients) = parse_fresh_intervals(input);
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
    let (mut ingredient_intervals, _) = parse_fresh_intervals(input);
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
