advent_of_code::solution!(9);

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

fn sort_pairs(areas: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let number_elements = areas.len();

    let mut pairs: Vec<(usize, usize)> =
        Vec::with_capacity(number_elements * (number_elements - 1) / 2);
    for index_1 in 0..number_elements {
        for index_2 in index_1 + 1..number_elements {
            pairs.push((index_1, index_2));
        }
    }

    pairs.sort_by(|&a, &b| areas[b.0][b.1].partial_cmp(&areas[a.0][a.1]).unwrap());

    pairs
}

pub fn part_one(input: &str) -> Option<i64> {
    let coordinates = parse_coordinates(input);
    let areas = pair_areas(&coordinates);
    let sorted_pairs = sort_pairs(&areas);
    let (index_1, index_2) = sorted_pairs[0];
    Some(areas[index_1][index_2])
}

pub fn part_two(input: &str) -> Option<i64> {
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
        assert_eq!(result, None);
    }
}
