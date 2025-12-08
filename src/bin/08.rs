use std::cmp::Reverse;
use std::collections::HashSet;

advent_of_code::solution!(8);

fn parse_coordinates(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split(",")
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect()
}

fn euclidean_distance_sq(vector_1: &Vec<i64>, vector_2: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    for index in 0..vector_1.len() {
        let diff = vector_1[index] - vector_2[index];
        result += diff * diff;
    }
    result // no need to compute root for comparisons
}

fn pair_distances(vectors: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    vectors
        .iter()
        .map(|vector_1| {
            vectors
                .iter()
                .map(|vector_2| euclidean_distance_sq(vector_1, vector_2))
                .collect()
        })
        .collect()
}

fn sort_pairs(distance_matrix: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let number_elements = distance_matrix.len();

    let mut pairs: Vec<(usize, usize)> =
        Vec::with_capacity(number_elements * (number_elements - 1) / 2);
    for index_1 in 0..number_elements {
        for index_2 in index_1 + 1..number_elements {
            pairs.push((index_1, index_2));
        }
    }

    pairs.sort_by(|&a, &b| {
        distance_matrix[a.0][a.1]
            .partial_cmp(&distance_matrix[b.0][b.1])
            .unwrap()
    });

    pairs
}

fn connect_circuits(coordinates: Vec<Vec<i64>>, all_connections: bool) -> i64 {
    let sorted_pairs = sort_pairs(&pair_distances(&coordinates));
    let number_connections: usize;
    if all_connections {
        number_connections = sorted_pairs.len();
    } else if coordinates.len() <= 20 {
        number_connections = 10;
    } else {
        number_connections = 1000;
    }
    let mut circuits: Vec<HashSet<&usize>> = Vec::new();
    for (point_1, point_2) in &sorted_pairs[..number_connections] {
        let mut intersecting: Vec<usize> = Vec::new();
        for index in 0..circuits.len() {
            if circuits[index].contains(&point_1) || circuits[index].contains(&point_2) {
                intersecting.push(index);
            }
        }
        let mut circuit: HashSet<&usize> = HashSet::from([point_1, point_2]);
        for index in intersecting.iter().rev() {
            circuit.extend(&circuits.remove(*index));
        }
        if circuit.len() == coordinates.len() { // reached part 2 condition
            return coordinates[*point_1][0] * coordinates[*point_2][0];
        }
        circuits.push(circuit);
    }
    circuits.sort_by_key(|circuit| Reverse(circuit.len()));
    circuits[..3]
        .iter()
        .map(|circuit| circuit.len())
        .product::<usize>() as i64
}

pub fn part_one(input: &str) -> Option<i64> {
    let coordinates = parse_coordinates(input);
    Some(connect_circuits(coordinates, false))
}

pub fn part_two(input: &str) -> Option<i64> {
    let coordinates = parse_coordinates(input);
    Some(connect_circuits(coordinates, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
