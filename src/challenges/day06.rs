use std::collections::HashSet;

use rayon::prelude::*;
use std::sync::Mutex;

use crate::utils::{self, load_input};

pub fn run() {
    let input = load_input(6);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    fn is_outside(&self, border: &Position) -> bool {
        self.x > border.x || self.y > border.y
    }

    fn on_obstacle(&self, obstacles: &Vec<Position>) -> bool {
        obstacles.iter().any(|obstacle| obstacle.x == self.x && obstacle.y == self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

#[derive(Debug, Clone)]
struct Guard{
    position: Position,
    direction: Direction,
    obstacles: Vec<Position>,
    border: Position,
    visited: Vec<Position>,
    is_loop: bool,
}

impl Guard {
    fn walk(&mut self) -> bool {
        let res = match self.direction {
            Direction::UP => self.up(),
            Direction::RIGHT => self.right(),
            Direction::DOWN => self.down(),
            Direction::LEFT => self.left(),
            _ => false
        };
        res
    }

    fn turn(&mut self) -> bool {
        match self.direction {
            Direction::UP => self.direction = Direction::RIGHT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::DOWN => self.direction = Direction::LEFT,
            Direction::LEFT => self.direction = Direction::UP,
            _ => eprintln!("Could not turn Direction::NONE")
        }

        self.walk()
    }

    fn has_visited(&self, position: &Position) -> bool {
        self.visited.contains(position) && self.visited.len() > 1
    }

    fn up(&mut self) -> bool {
        if self.position.y as i32 - 1 < 0 { return false; }
        let new_pos = Position{ x: self.position.x, y: self.position.y - 1, direction: Direction::UP };
        if new_pos.is_outside(&self.border) || self.has_visited(&new_pos) {
            self.is_loop = self.has_visited(&new_pos);
            return false;
        } else if new_pos.on_obstacle(&self.obstacles) {
            self.turn()
        } else {
            self.position = new_pos;
            self.visited.push(self.position.clone());
            true
        }
    }

    fn down(&mut self) -> bool {
        let new_pos = Position{ x: self.position.x, y: self.position.y + 1, direction: Direction::DOWN };
        if new_pos.is_outside(&self.border) || self.has_visited(&new_pos) {
            self.is_loop = self.has_visited(&new_pos);
            return false;
        } else if new_pos.on_obstacle(&self.obstacles) {
            self.turn()
        } else {
            self.position = new_pos;
            self.visited.push(self.position.clone());
            true
        }
    }
    fn left(&mut self) -> bool {
        if self.position.x as i32 - 1 < 0 { return false; }
        let new_pos = Position{ x: self.position.x - 1, y: self.position.y, direction: Direction::LEFT };
        if new_pos.is_outside(&self.border) || self.has_visited(&new_pos) {
            self.is_loop = self.has_visited(&new_pos);
            return false;
        } else if new_pos.on_obstacle(&self.obstacles) {
            self.turn()
        } else {
            self.position = new_pos;
            self.visited.push(self.position.clone());
            true
        }
    }
    fn right(&mut self) -> bool {
        let new_pos = Position{ x: self.position.x + 1, y: self.position.y, direction: Direction::RIGHT };
        if new_pos.is_outside(&self.border) || self.has_visited(&new_pos) {
            self.is_loop = self.has_visited(&new_pos);
            return false;
        } else if new_pos.on_obstacle(&self.obstacles) {
            self.turn()
        } else {
            self.position = new_pos;
            self.visited.push(self.position.clone());
            true
        }
    }

}

fn get_obstacles(rows: &[&str]) -> Vec<Position> {
    let mut obstacles: Vec<Position> = vec![];
    rows.iter().enumerate().for_each(|(j, row)| {
        row.chars().enumerate().for_each(|(i, letter)| {
            if letter == '#' {
                obstacles.push(Position{ x: i, y: j, direction: Direction::NONE });
            }
        });
    });
    obstacles
}

fn get_unique_positions(positions: &Vec<Position>) -> Vec<(usize, usize)> {
    let mut unique_positions = HashSet::new();
    let result: Vec<(usize, usize)> = positions.iter().filter_map(|pos| {
        let pair = (pos.x, pos.y);
        if unique_positions.insert(pair) {
            Some(pair)
        } else {
            None
        }
    }).collect();
    result
}

fn solve_part1(input: &str) -> u32 {
    let rows: Vec<&str> = utils::parse_input(&input);
    let start_row_index = rows.iter().position(|row| row.contains('^')).expect("Could not find y position of guard");
    let start_col_index =  rows[start_row_index].chars().position(|letter| letter == '^').expect("Could not find x position of guard");
    let start_position = Position { x: start_col_index, y: start_row_index, direction: Direction::UP };
    let obstacles = get_obstacles(&rows);

    let mut guard = Guard {
        visited: vec![start_position.clone()],
        obstacles,
        position: start_position,
        border: Position { x: rows.first().map_or(0, |row| row.len() - 1), y: rows.len() - 1, direction: Direction::NONE },
        direction: Direction::UP,
        is_loop: false,
    };
    while guard.walk() { }
    let result = get_unique_positions(&guard.visited);
    result.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    let rows: Vec<&str> = utils::parse_input(&input);
    let start_row_index = rows.iter().position(|row| row.contains('^')).expect("Could not find y position of guard");
    let start_col_index =  rows[start_row_index].chars().position(|letter| letter == '^').expect("Could not find x position of guard");
    let start_position = Position { x: start_col_index, y: start_row_index, direction: Direction::UP };

    let mut normal_guard = Guard {
        visited: vec![start_position.clone()],
        obstacles: get_obstacles(&rows),
        position: start_position.clone(),
        border: Position { x: rows.first().map_or(0, |row| row.len() - 1), y: rows.len() - 1, direction: Direction::NONE },
        direction: Direction::UP,
        is_loop: false,
    };
    while normal_guard.walk() { }


    let loop_obstacles: Mutex<Vec<Position>> = Mutex::new(vec![]);
    normal_guard.visited.par_iter().for_each(|obs_pos| {
        let new_obstacle_position = Position { x: obs_pos.x, y: obs_pos.y, direction: Direction::NONE };
        let mut obstacles = get_obstacles(&rows);
        if new_obstacle_position.on_obstacle(&obstacles)
            || (new_obstacle_position.x == start_position.x && new_obstacle_position.y == start_position.y)
            { return; }
        obstacles.push(new_obstacle_position.clone());
        let mut guard = Guard {
            visited: vec![start_position.clone()],
            obstacles,
            position: start_position.to_owned(),
            border: Position { x: rows.first().map_or(0, |row| row.len() - 1), y: rows.len() - 1, direction: Direction::NONE },
            direction: Direction::UP,
            is_loop: false
        };

        while guard.walk() {
        }

        if guard.is_loop {
            loop_obstacles.lock().unwrap().push(new_obstacle_position.clone());
        }

    });
    get_unique_positions(&loop_obstacles.into_inner().unwrap()).len() as u32
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 6);
    }

}
// 3 6 check
// 6 7 check
// 3 8 check
// 1 8 check
// 6 7 check
// 7 7 check
// 7 9 check
