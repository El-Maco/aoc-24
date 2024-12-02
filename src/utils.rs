use std::fs;

pub fn load_input(day: u32) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(path).expect("Failed to read input file")
}
