extern crate queues;
use std::collections::HashSet;

use queues::*;

use crate::utils::{load_input, parse_input};

pub fn run() {
    let input = load_input(10);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

type Trail = Vec<MapPosition>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct MapPosition {
    x: i32,
    y: i32,
    height: Option<i32>,
}

impl MapPosition {
    fn new(x: i32, y: i32, height: Option<i32>) -> Self {
        MapPosition { x, y, height }
    }
}

fn get_valid_neighbors(position: &MapPosition, rows: &Vec<Vec<MapPosition>>) -> Vec<MapPosition> {
    let mut valid_neighbors: Vec<MapPosition> = vec![];
    // Right
    if let Some(row) = rows.get(position.y as usize) {
        if let Some(neighbor) = row.get((position.x + 1) as usize) {
            if Some(neighbor.height) > Some(position.height) {
                valid_neighbors.push(neighbor.clone());
            }
        }
    }

    // Down
    if let Some(row) = rows.get((position.y + 1) as usize) {
        if let Some(neighbor) = row.get(position.x as usize) {
            if Some(neighbor.height) > Some(position.height) {
                valid_neighbors.push(neighbor.clone());
            }
        }
    }

    // up
    if position.y - 1 >= 0 {
        if let Some(row) = rows.get((position.y - 1) as usize) {
            if let Some(neighbor) = row.get(position.x as usize) {
                if Some(neighbor.height) > Some(position.height) {
                    valid_neighbors.push(neighbor.clone());
                }
            }
        }
    }

    // Left
    if position.x - 1 >= 0 {
        if let Some(row) = rows.get(position.y as usize) {
            if let Some(neighbor) = row.get((position.x - 1) as usize) {
                if Some(neighbor.height) > Some(position.height) {
                    valid_neighbors.push(neighbor.clone());
                }
            }
        }
    }
    valid_neighbors
}

fn find_trails(x: i32, y: i32, rows: &Vec<Vec<MapPosition>>) -> HashSet<MapPosition> {
    let trailhead = rows[y as usize][x as usize]; // Should be a 0
    let mut trails: Queue<Trail> = queue![];
    let root: Trail = vec![trailhead];
    trails.add(root).unwrap();
    let mut found_trails: HashSet<MapPosition> = HashSet::new();

    loop {
        let trail: Trail = match trails.remove() {
            Ok(trail) => trail,
            Err(e) => {
                break;
            }
        };
        let position = trail.last().unwrap();
        if position.height.unwrap() == 9 && trail.len() == 10 {
            found_trails.insert(*position);
            continue;
        }
        let valid_neighbors: Vec<MapPosition> = get_valid_neighbors(position, &rows);
        if valid_neighbors.len() == 0 {
            continue;
        }
        valid_neighbors.iter().for_each(|neighbor| {
            let mut new_trail: Trail = trail.clone();
            new_trail.push(*neighbor);
            trails.add(new_trail).unwrap();
        });
    }
    found_trails
}

fn find_unique_trails(x: i32, y: i32, rows: &Vec<Vec<MapPosition>>) -> Vec<Trail> {
    let trailhead = rows[y as usize][x as usize]; // Should be a 0
    let mut trails: Queue<Trail> = queue![];
    let root: Trail = vec![trailhead];
    trails.add(root).unwrap();
    let mut found_trails: Vec<Trail> = vec![];

    loop {
        let trail: Trail = match trails.remove() {
            Ok(trail) => trail,
            Err(e) => {
                break;
            }
        };
        let position = trail.last().unwrap();
        if position.height.unwrap() == 9 && trail.len() == 10 {
            found_trails.push(trail);
            continue;
        }
        let valid_neighbors: Vec<MapPosition> = get_valid_neighbors(position, &rows);
        if valid_neighbors.len() == 0 {
            continue;
        }
        valid_neighbors.iter().for_each(|neighbor| {
            let mut new_trail: Trail = trail.clone();
            new_trail.push(*neighbor);
            trails.add(new_trail).unwrap();
        });
    }
    found_trails
}

fn solve_part1(input: &str) -> i32 {
    let row = parse_input(input);
    let map: Vec<Vec<MapPosition>> = row
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, ch)| {
                    MapPosition::new(
                        x as i32,
                        y as i32,
                        Some(ch.to_digit(10).expect("Failed to parse digit") as i32),
                    )
                })
                .collect()
        })
        .collect();

    let mut total_score: i32 = 0;
    map.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, d)| {
            if d.height.unwrap() == 0 {
                let trails = find_trails(i as i32, j as i32, &map);
                let trailhead_score = trails.len() as i32;
                total_score += trailhead_score;
            }
        });
    });

    total_score
}

fn solve_part2(input: &str) -> i32 {
    let row = parse_input(input);
    let map: Vec<Vec<MapPosition>> = row
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, ch)| {
                    MapPosition::new(
                        x as i32,
                        y as i32,
                        Some(ch.to_digit(10).expect("Failed to parse digit") as i32),
                    )
                })
                .collect()
        })
        .collect();

    let mut total_score: i32 = 0;
    map.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, d)| {
            if d.height.unwrap() == 0 {
                let trails = find_unique_trails(i as i32, j as i32, &map);
                let trailhead_score = trails.len() as i32;
                total_score += trailhead_score;
            }
        });
    });

    total_score
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 36);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 81);
    }
}
