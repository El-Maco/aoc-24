use core::panic;
use itertools::Itertools;
use crate::utils::{load_input, parse_input};

pub fn run() {
    let input = load_input(7);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn split_row(row: &str) -> (u64, Vec<u64>) {
    let v: Vec<&str> = row.split(':').collect();
    let Ok(result) = v[0].trim().parse::<u64>() else { panic!("{:?} failed", v[0]);};
    let operands: Vec<u64> = v[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    (result, operands)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Add,
}

#[derive(Debug, Clone, Copy)]
enum OperatorTwo {
    Mul,
    Add,
    Cat,
}

fn apply_operator(a: u64, b: u64, operator: &Operator) -> u64 {
    match operator {
        Operator::Mul => a * b,
        Operator::Add => a + b,
    }
}

fn apply_operator_two(a: u64, b: u64, operator: &OperatorTwo) -> u64 {
    match operator {
        OperatorTwo::Mul => a * b,
        OperatorTwo::Add => a + b,
        OperatorTwo::Cat => {
            let res = a.to_string() + &b.to_string();
            res.parse::<u64>().unwrap()
        }
    }
}

fn evaluate((result, operands): &&(u64, Vec<u64>)) -> bool {
    let operators = vec![Operator::Mul, Operator::Add];
    let combinations: Vec<Vec<Operator>>= vec![operators; operands.len() - 1]
        .into_iter().multi_cartesian_product().collect();

    combinations.iter().any(|combination| {
        let mut tot = 0;
        for (i, v) in operands.windows(2).enumerate() {
            let a = if i == 0 { v[0] } else { tot };
            let b = v[1];
            let operator: Operator = combination[i];
            tot = apply_operator(a, b, &operator);
        }
        tot == *result
    })
}

fn evaluate_two((result, operands): &&(u64, Vec<u64>)) -> bool {
    let operators = vec![OperatorTwo::Mul, OperatorTwo::Add, OperatorTwo::Cat];
    let combinations: Vec<Vec<OperatorTwo>>= vec![operators; operands.len() - 1]
        .into_iter().multi_cartesian_product().collect();

    combinations.iter().any(|combination| {
        let mut tot = 0;
        for (i, v) in operands.windows(2).enumerate() {
            let a = if i == 0 { v[0] } else { tot };
            let b = v[1];
            let operator: OperatorTwo = combination[i];
            tot = apply_operator_two(a, b, &operator);
        }
        tot == *result
    })
}

fn solve_part1(input: &str) -> u64 {
    let rows = parse_input(input);
    let rows: Vec<(u64, Vec<u64>)> = rows.iter().map(|row| split_row(row)).collect();
    rows.iter()
        .filter(evaluate)
        .map(|(r, _)| r)
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let rows = parse_input(input);
    let rows: Vec<(u64, Vec<u64>)> = rows.iter().map(|row| split_row(row)).collect();
    rows.iter()
        .filter(evaluate_two)
        .map(|(r, _)| r)
        .sum()
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 11387);
    }
}
