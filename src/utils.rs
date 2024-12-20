use std::fs;

pub fn load_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(path).expect("Failed to read input file")
}

pub fn parse_input(input: &str) -> Vec<&str> {
    let rows: Vec<&str> = input.trim().split("\n").collect();
    rows
}

pub fn parse_line(row: &str) -> Vec<i32> {
    let elements: Vec<&str> = row.split_whitespace().collect();
    elements.iter().map(|v| v.parse().unwrap()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn is_contained(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width && self.y < height
    }

    // pub fn distance_to(&self, position: &Position) -> f64 {
    //     let (a, b) = ((position.x - self.x) as f64, (position.y - self.y) as f64);
    //     let distance = (a.powi(2) + b.powi(2)).sqrt();
    //     distance
    // }

    pub fn add(&self, position: &Position) -> Position {
        Position::new(self.x + position.x, self.y + position.y)
    }

    // pub fn exists_in(&self, positions: &Vec<Position>) -> bool {
    //     positions.iter().any(|pos| self.eq(pos))
    // }

    // pub fn eq(&self, position: &Position) -> bool {
    //     self.x == position.x &&
    //         self.y == position.y
    // }

    pub fn diff(&self, position: &Position) -> Position {
        Position::new(position.x - self.x, position.y - self.y)
    }
}
