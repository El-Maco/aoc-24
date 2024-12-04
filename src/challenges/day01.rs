use std::collections::HashMap;

use crate::utils::load_input;

pub fn run() {
    let input = load_input(1);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn solve_part1(input: &str) -> i32 {
    let rows: Vec<&str> = input.trim().split("\n").collect();
    let mut a_vec: Vec<i32> = vec![];
    let mut b_vec: Vec<i32> = vec![];

    for row in rows.iter() {
        let splitted: Vec<&str> = row.split_whitespace().collect();
        a_vec.push(splitted[0].parse().unwrap());
        b_vec.push(splitted[1].parse().unwrap());
    }
    a_vec.sort();
    b_vec.sort();

    a_vec.iter().zip(b_vec.iter()).map(|(a, b)| (b - a).abs()).sum()
}

fn solve_part2(input: &str) -> i32 {
    let rows: Vec<&str> = input.trim().split("\n").collect();
    let mut a_vec: Vec<i32> = vec![];
    let mut b_map: HashMap<i32, i32> = HashMap::new();

    for row in rows.iter() {
        let splitted: Vec<&str> = row.split_whitespace().collect();

        let a: i32 = splitted[0].parse().unwrap();
        a_vec.push(a);

        let b: i32 = splitted[1].parse().unwrap();
        if let Some(count) = b_map.get(&b) {
            b_map.insert(b, count + 1);
        } else {
            b_map.insert(b, 1);
        }
    }

    a_vec
        .iter()
        .map(|a| a * b_map.get(&a).unwrap_or(&0i32))
        .sum()
}
