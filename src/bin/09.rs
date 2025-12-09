advent_of_code::solution!(9);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_coordinates(input: &str) -> Vec<(i64, i64)> {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split_line = line.split(",");
            let value_1 = split_line.next().unwrap().parse().unwrap();
            let value_2 = split_line.next().unwrap().parse().unwrap();
            (value_1, value_2)
        })
        .collect()
}

fn rectangle_area(corner_1: &(i64, i64), corner_2: &(i64, i64)) -> i64 {
    ((corner_1.0 - corner_2.0).abs() + 1) * ((corner_1.1 - corner_2.1).abs() + 1)
}

fn pair_areas(vectors: &Vec<(i64, i64)>) -> Vec<Vec<i64>> {
    vectors
        .iter()
        .map(|vector_1| {
            vectors
                .iter()
                .map(|vector_2| rectangle_area(vector_1, vector_2))
                .collect()
        })
        .collect()
}

fn get_pairs(number_elements: usize) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> =
        Vec::with_capacity(number_elements * (number_elements - 1) / 2);
    for index_1 in 0..number_elements {
        for index_2 in index_1 + 1..number_elements {
            pairs.push((index_1, index_2));
        }
    }
    pairs
}

fn sort_pairs(areas: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let mut pairs = get_pairs(areas.len());
    pairs.sort_by(|&a, &b| areas[b.0][b.1].partial_cmp(&areas[a.0][a.1]).unwrap());

    pairs
}

fn get_line(point_1: (i64, i64), point_2: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut border = HashSet::from([point_1, point_2]);
    if point_1.0 == point_2.0 {
        let min_value = point_1.1.min(point_2.1);
        let max_value = point_1.1.max(point_2.1);
        for value in min_value..max_value {
            border.insert((point_1.0, value));
        }
    } else if point_1.1 == point_2.1 {
        let min_value = point_1.0.min(point_2.0);
        let max_value = point_1.0.max(point_2.0);
        for value in min_value..max_value {
            border.insert((value, point_1.1));
        }
    } else {
        panic!("Tried to compute line between points not in line")
    }
    border
}

fn get_border(coordinates: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut border = HashSet::new();
    let total = coordinates.len();
    for index in 0..total {
        let point_1 = coordinates[index];
        let point_2 = coordinates[(index + 1) % coordinates.len()];
        border.extend(get_line(point_1, point_2));
    }
    border
}

fn square_vertices(corner_1: (i64, i64), corner_2: (i64, i64)) -> Vec<(i64, i64)> {
    Vec::from([
        corner_1,
        (corner_1.0, corner_2.1),
        corner_2,
        (corner_2.0, corner_1.1),
    ])
}

fn square_border(corner_1: (i64, i64), corner_2: (i64, i64)) -> HashSet<(i64, i64)> {
    get_border(&square_vertices(corner_1, corner_2))
}

fn fill_outside(
    border: &HashSet<(i64, i64)>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
) -> HashSet<(i64, i64)> {
    let mut outside = HashSet::new();
    let mut queue = Vec::from([(min_x, min_y)]);
    while queue.len() > 0 {
        let (x, y) = queue.pop().unwrap();
        let mut next_points = Vec::new();
        if x + 1 <= max_x {
            next_points.push((x + 1, y));
        }
        if y + 1 <= max_y {
            next_points.push((x, y + 1));
        }
        if x - 1 >= min_x {
            next_points.push((x - 1, y));
        }
        if y - 1 >= min_y {
            next_points.push((x, y - 1));
        }
        for point in next_points {
            if !border.contains(&point) {
                if outside.insert(point) {
                    queue.push(point)
                }
            }
        }
    }
    outside
}

fn compressed_coordinates(coordinates: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut x_coordinates = HashMap::new();
    let mut y_coordinates = HashMap::new();
    let mut all_x: Vec<&i64> = coordinates.iter().map(|(x, _)| x).unique().collect();
    all_x.sort();
    for (index, value) in all_x.iter().enumerate() {
        x_coordinates.insert(*value, 2 * index as i64);
    }
    let mut all_y: Vec<&i64> = coordinates.iter().map(|(_, y)| y).unique().collect();
    all_y.sort();
    for (index, value) in all_y.iter().enumerate() {
        y_coordinates.insert(*value, 2 * index as i64);
    }
    coordinates
        .iter()
        .map(|(x, y)| (x_coordinates[x], y_coordinates[y]))
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let coordinates = parse_coordinates(input);
    let areas = pair_areas(&coordinates);
    let sorted_pairs = sort_pairs(&areas);
    let (index_1, index_2) = sorted_pairs[0];
    Some(areas[index_1][index_2])
}

pub fn part_two(input: &str) -> Option<i64> {
    let coordinates = parse_coordinates(input);
    let small_coordinates = compressed_coordinates(&coordinates);
    let border = get_border(&small_coordinates);
    let min_x = small_coordinates.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = small_coordinates.iter().map(|(x, _)| x).max().unwrap() + 1;
    let min_y = small_coordinates.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = small_coordinates.iter().map(|(_, y)| y).max().unwrap() + 1;
    let outside = fill_outside(&border, min_x, max_x, min_y, max_y);
    let areas = pair_areas(&coordinates);
    let sorted_pairs = sort_pairs(&areas);
    for (index_1, index_2) in sorted_pairs {
        let vertices = square_vertices(small_coordinates[index_1], small_coordinates[index_2]);
        // this next check reduces runtime massively
        if vertices.into_iter().any(|vertex| outside.contains(&vertex)) {continue;}
        if square_border(small_coordinates[index_1], small_coordinates[index_2]).is_disjoint(&outside) {
            return Some(rectangle_area(&coordinates[index_1], &coordinates[index_2]))
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
