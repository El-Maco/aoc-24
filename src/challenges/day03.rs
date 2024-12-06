use regex::Regex;

use crate::utils::load_input;

pub fn run() {
    let input = load_input(3);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn get_valid_muls(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    re.find_iter(input)
        .map(|m| extract_factors(m.as_str()))
        .collect()
}

fn extract_factors(text: &str) -> (u32, u32) {
    let re = Regex::new(r"\((?<first>.*),(?<second>.*)\)").unwrap();
    let Some(caps) = re.captures(text) else {
        return (0, 0);
    };
    let first: u32 = caps["first"].parse().unwrap();
    let second: u32 = caps["second"].parse().unwrap();
    (first, second)
}

fn find_dos(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(^|do\(\)).*?don't\(\)").unwrap();
    re.find_iter(input).map(|m| m.as_str()).collect()
}

fn solve_part1(input: &str) -> u32 {
    get_valid_muls(input).iter().map(|(a, b)| a * b).sum()
}

fn solve_part2(input: &str) -> u32 {
    let oneline = input.replace("\n", "");
    let dos = find_dos(&oneline);
    dos.iter()
        .map(|curr_do| {
            let tot: u32 = get_valid_muls(curr_do).iter().map(|(a, b)| a * b).sum();
            tot
        })
        .sum()
}
