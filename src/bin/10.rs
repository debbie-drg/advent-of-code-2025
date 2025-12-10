advent_of_code::solution!(10);

use microlp::{LinearExpr, OptimizationDirection, Problem, ComparisonOp};
use std::collections::HashSet;
use std::ops::{BitXor, Index};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BooleanVector(Vec<bool>);

impl BitXor for BooleanVector {
    type Output = Self;

    fn bitxor(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        Self(lhs.iter().zip(rhs.iter()).map(|(x, y)| *x ^ *y).collect())
    }
}

impl Index<usize> for BooleanVector {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        let Self(vec) = self;
        &vec[index]
    }
}

impl BooleanVector {
    pub const fn len(&self) -> usize {
        let Self(vec) = self;
        vec.len()
    }
}

fn parse_machine(input: &str) -> (BooleanVector, Vec<BooleanVector>, Vec<u64>) {
    let (target, rest) = input.split_once(" ").unwrap();
    let mut target_vec = Vec::new();
    for char in target.chars() {
        match char {
            '.' => target_vec.push(false),
            '#' => target_vec.push(true),
            _ => (),
        }
    }
    let num_buttons = target_vec.len();
    let target_vec = BooleanVector(target_vec);
    let (buttons, joltajes) = rest.rsplit_once(" ").unwrap();
    let joltages = joltajes
        .strip_suffix("}")
        .unwrap()
        .strip_prefix("{")
        .unwrap()
        .split(",")
        .map(|element| element.parse().unwrap())
        .collect();
    let buttons = buttons
        .split(" ")
        .map(|button| {
            button
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|element| element.parse().unwrap())
                .collect()
        })
        .map(|buttons: Vec<u64>| {
            BooleanVector({
                (0..num_buttons as u64)
                    .map(|element| buttons.contains(&element))
                    .collect()
            })
        })
        .collect();
    (target_vec, buttons, joltages)
}

fn parse_machines(input: &str) -> Vec<(BooleanVector, Vec<BooleanVector>, Vec<u64>)> {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| parse_machine(line))
        .collect()
}

fn times_to_turn_on(machine: &(BooleanVector, Vec<BooleanVector>, Vec<u64>)) -> u64 {
    let (goal, buttons, _) = machine;
    let start_state = BooleanVector(vec![false; goal.len()]);
    let mut states = HashSet::from([start_state]);
    let mut next_states = HashSet::new();
    let mut count = 0;
    loop {
        count += 1;
        for state in states {
            for button in buttons.clone() {
                let next_state = button ^ state.clone();
                if &next_state == goal {
                    return count;
                }
                next_states.insert(next_state);
            }
        }
        states = next_states;
        next_states = HashSet::new();
    }
}

fn times_to_match_joltage(machine: &(BooleanVector, Vec<BooleanVector>, Vec<u64>)) -> u64 {
    let (_, buttons, joltage) = machine;
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let mut vars = Vec::new();
    for _ in 0..buttons.len() {
        vars.push(problem.add_integer_var(1.0, (0, i32::MAX)));
    }
    for constraint in 0..joltage.len() {
        let mut equation = LinearExpr::empty();
        for variable in 0..buttons.len() {
            if buttons[variable][constraint] {
                equation.add(vars[variable], 1.0);
            }
        }
        problem.add_constraint(
            equation,
            ComparisonOp::Eq,
            joltage[constraint] as f64,
        );
    }
    problem.solve().unwrap().objective().round() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(
        machines
            .iter()
            .map(|machine| times_to_turn_on(machine))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(
        machines
            .iter()
            .map(|machine| times_to_match_joltage(machine))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
