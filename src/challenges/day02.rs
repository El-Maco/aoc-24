use crate::utils::{load_input, parse_input, parse_line};

pub fn run() {
    let input = load_input(2);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn all_increasing(row: &[i32]) -> bool {
    row.is_sorted_by(|a, b| a < b && *b <= a + 3)
}

fn all_decreasing(row: &[i32]) -> bool {
    row.is_sorted_by(|a, b| a - 3 <= *b && b < a)
}

fn is_safe(row: &[i32]) -> bool {
    all_increasing(row) || all_decreasing(row)
}

fn skip_index(row: &[i32], index: usize) -> Vec<i32> {
    row.iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, &v)| v)
        .collect()
}

fn solve_part1(input: &str) -> i32 {
    let rows = parse_input(&input);
    rows.iter()
        .map(|line| {
            let row = parse_line(line);
            let safe = is_safe(&row);
            safe
        })
        .filter(|b| *b)
        .count() as i32
}

fn solve_part2(input: &str) -> i32 {
    let rows = parse_input(&input);
    rows.iter()
        .map(|line| {
            let row = parse_line(line);
            let safe = is_safe(&row);
            if !safe {
                for i in 0..row.len() {
                    let new_row = skip_index(&row, i);
                    if is_safe(&new_row) {
                        return true;
                    }
                }
            }
            safe
        })
        .filter(|b| *b)
        .count() as i32
}
