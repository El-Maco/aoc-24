use std::collections::HashMap;
// use rayon::prelude::*;

use crate::utils::{load_input, parse_input};

pub fn run() {
    let input = load_input(5);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn split_at_empty_element(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parsed = parse_input(input);
    let i = parsed.iter().position(|s| s.is_empty()).unwrap();
    (parsed[..i].to_vec(), parsed[i+1..].to_vec())
}

fn follows_rules(updates: &[&str], rules: &[(&str, &str)]) -> bool {
    updates.iter().enumerate().all(
        |(first_pos, &update)|
        {
            rules.iter().all(|&(first, second)| {
                if update == first {
                    let second_is_after = match updates.iter().position(|update| *update == second)  {
                        Some(second_pos) => second_pos > first_pos,
                        None => true
                    };
                    second_is_after
                } else {
                    true
                }
            })
        }
        )
}

fn solve_part1(input: &str) -> u32 {
    let (rules_line, updates_line) = split_at_empty_element(input);
    let rules: Vec<(&str, &str)> = rules_line.iter().map(|&rule| {
        let v = rule.split('|').collect::<Vec<&str>>();
        (v[0], v[1])
    }).collect();
    updates_line.iter().map(|&u|  u.split(',').map(|s| s).collect::<Vec<&str>>())
        .filter(|updates| follows_rules(&updates, &rules))
        .map(|updates| updates.get(updates.len() / 2).unwrap().parse::<u32>().unwrap())
        .sum()
}

fn create_rules_map(rules: &[(&str, &str)]) -> HashMap<u32, Vec<u32>> {
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    rules.iter().for_each(|&(first, second)| {
        let key: u32 = first.parse().unwrap();
        let value: u32 = second.parse().unwrap();
        if let Some(arr) = rules_map.get_mut(&key) {
            arr.push(value);
        } else {
            rules_map.insert(key, vec![value]);
        }
    });
    rules_map
}

fn reorder(updates: &mut Vec<&str>, rules_map: &HashMap<u32, Vec<u32>>) {
    let orig_updates = updates.clone();
    let mut swapped: bool = false;
    orig_updates.iter().enumerate()
        .for_each(|(first_pos, update)| {
            if swapped { return; }
            let update_as_int: u32 = update.parse().unwrap();
            if let Some(updates_after) = rules_map.get(&update_as_int) {
                updates_after.iter().for_each(|&update_after| {
                    if swapped { return; }
                    if let Some(second_pos) = orig_updates.iter().position(|o| o.parse().unwrap_or(0) == update_after) {
                        if second_pos <= first_pos {
                            updates.swap(first_pos, second_pos);
                            swapped = true;
                            return;
                        }
                    }
                });
            } else {
            }

        });
}

fn solve_part2(input: &str) -> u32 {
    let (rules_line, updates_line) = split_at_empty_element(input);
    let rules: Vec<(&str, &str)> = rules_line.iter().map(|&rule| {
        let v = rule.split('|').collect::<Vec<&str>>();
        (v[0], v[1])
    }).collect();

    let rules_map = create_rules_map(&rules);

    updates_line.iter()
        .map(|&u|  u.split(',').map(|s| s).collect::<Vec<&str>>())
        .filter(|updates| !follows_rules(&updates, &rules))
        .map(|updates| {
            let mut cpy = updates.clone();
            // let mut irs = Irs::default();
            // let mut rng = StepRng::new(2, 13);
            while !follows_rules(&cpy, &rules) {
                // irs.shuffle(&mut cpy, &mut rng).unwrap_or(());
                reorder(&mut cpy, &rules_map);
            }
            cpy
        })
        .map(|updates| updates.get(updates.len() / 2).unwrap().parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";


    use super::*;

    #[test]
    fn test_get_rulse_and_updates() {
        let (rules, updates) = split_at_empty_element(TEST_INPUT);
        let result_rules: Vec<&str> = vec!["47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29", "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61", "47|29", "75|13", "53|13"];
        let result_updates: Vec<&str> = vec!["75,47,61,53,29", "97,61,53,29,13", "75,29,13", "75,97,47,61,53", "61,13,29", "97,13,75,29,47"];
        assert_eq!(rules, result_rules);
        assert_eq!(updates, result_updates);
    }

    #[test]
    fn test_solve_part1() {
        let result = solve_part1(TEST_INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_solve_part2() {
        let result = solve_part2(TEST_INPUT);
        assert_eq!(result, 123);
    }

}

