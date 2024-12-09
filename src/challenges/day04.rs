use regex::Regex;

use crate::utils::{load_input, parse_input};

pub fn run() {
    let input = load_input(4);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn find_horizontal_xmas(row: &&str) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();
    let count = re.find_iter(row).count() as u32;

    let re = Regex::new(r"SAMX").unwrap();
    let count_rev = re.find_iter(row).count() as u32;
    count + count_rev
}

fn rows_to_string(rows: &[&str]) -> String {
    let mut output: String = String::new();
    rows.iter().for_each(|row| {
        output.push_str(row.trim());
        output.push(' ');
    });
    output
}

fn cols_to_string(rows: &[&str]) -> String {
    let mut output: String = String::new();
    let width = rows.first().map_or(0, |row| row.len());
    for i in 0..width {
        rows.iter().for_each(|row| match row.trim().chars().nth(i) {
            Some(letter) => {
                output.push(letter);
            }
            None => output.push(' '),
        });
        output.push(' ');
    }
    output
}

fn diagonals_to_string(rows: &[&str]) -> String {
    let height = rows.len();

    // Left to right
    let mut output: String = String::new();
    for i in 0..height {
        for (j, row) in rows[i..].iter().enumerate() {
            if let Some(letter) = row.trim().chars().nth(j) {
                output.push(letter);
            } else {
                output.push(' ');
            }
        }
        output.push(' ');
    }
    let width = rows.first().map_or(0, |row| row.len());
    for i in 1..width {
        for (j, row) in rows.iter().enumerate() {
            if let Some(letter) = row.trim().chars().nth(i + j) {
                output.push(letter);
            } else {
                output.push(' ');
            }
        }
        output.push(' ');
    }
    let reversed_rows: Vec<String> = rows
        .iter()
        .map(|row| {
            let new_row: String = row.chars().rev().collect();
            new_row
        })
        .collect();
    // Left to right
    for i in 0..height {
        for (j, row) in reversed_rows[i..].iter().enumerate() {
            if let Some(letter) = row.trim().chars().nth(j) {
                output.push(letter);
            } else {
                output.push(' ');
            }
        }
        output.push(' ');
    }
    let width = reversed_rows.first().map_or(0, |row| row.len());
    for i in 1..width {
        for (j, row) in reversed_rows.iter().enumerate() {
            if let Some(letter) = row.trim().chars().nth(i + j) {
                output.push(letter);
            } else {
                output.push(' ');
            }
        }
        output.push(' ');
    }

    output
}

fn solve_part1(input: &str) -> u32 {
    let rows: Vec<&str> = parse_input(input);
    let horizontal_string = rows_to_string(&rows);
    let vertical_string = cols_to_string(&rows);
    let diagonal_string = diagonals_to_string(&rows);
    find_horizontal_xmas(&&horizontal_string[..])
        + find_horizontal_xmas(&&vertical_string[..])
        + find_horizontal_xmas(&&diagonal_string[..])
}

fn is_xmas(i: usize, j: usize, rows: &[&str]) -> bool {
    let Some(prev_row) = rows.get(i - 1) else {
        return false;
    };
    let Some(next_row) = rows.get(i + 1) else {
        return false;
    };

    let left_diag = (prev_row.trim().chars().nth(j - 1), next_row.trim().chars().nth(j + 1));
    let left_diag_mas = match left_diag {
        (Some(x0), Some(x1)) => (x0 == 'M' && x1 == 'S') || (x0 == 'S' && x1 == 'M'),
        _ => false,
    };

    let right_diag = (prev_row.trim().chars().nth(j + 1), next_row.trim().chars().nth(j - 1));
    let right_diag_mas = match right_diag {
        (Some(x0), Some(x1)) => (x0 == 'M' && x1 == 'S') || (x0 == 'S' && x1 == 'M'),
        _ => false,
    };

    left_diag_mas && right_diag_mas
}

fn solve_part2(input: &str) -> u32 {
    let rows: Vec<&str> = parse_input(input);
    let mut count = 0;
    for (i, row) in rows.iter().enumerate() {
        if i == 0 {
            continue;
        }
        for (j, letter) in row.trim().chars().enumerate() {
            if j == 0 {
                continue;
            }
            if letter == 'A' {
                if is_xmas(i, j, &rows[..]) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &'static str = r"MMMSXXMASM
                                        MSAMXMSMSA
                                        AMXSXMAAMM
                                        MSAMASMSMX
                                        XMASAMXAMM
                                        XXAMMXXAMA
                                        SMSMSASXSS
                                        SAXAMASAAA
                                        MAMMMXMMMM
                                        MXMXAXMASX";

    const ROWS_PART2_TEST: &str = "MMMS
                                   MASD
                                   SSSM
                                   SLMS";

    const ROWS_ONE_ROW_ONE_COL: &str = "XMAS
                                        MMAD
                                        AJAL
                                        SLKS";
    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 18)
    }

    #[test]
    fn test_cols_to_string() {
        let rows = parse_input(ROWS_ONE_ROW_ONE_COL);
        let the_string = cols_to_string(&rows);
        assert_eq!(the_string, "XMAS MMJL AAAK SDLS ");
        let xmases = find_horizontal_xmas(&&the_string[..]);
        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_rows_to_string() {
        let rows = parse_input(ROWS_ONE_ROW_ONE_COL);
        let the_string = rows_to_string(&rows);
        assert_eq!(the_string, "XMAS MMAD AJAL SLKS ");
        let xmases = find_horizontal_xmas(&&the_string[..]);
        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_diagonals_to_string() {
        let rows = parse_input(ROWS_ONE_ROW_ONE_COL);
        let the_string = diagonals_to_string(&rows);
        let arr: Vec<&str> = the_string.split_whitespace().collect();
        let result: &str = "XMAS MJK AL S MAL AD S SAJS DAL LK S AMA MM X";
        assert_eq!(arr.len(), result.split(" ").collect::<Vec<&str>>().len());
        assert_eq!(arr.join(" "), result);
        let xmases = find_horizontal_xmas(&&the_string[..]);
        assert_eq!(xmases, 1);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(ROWS_PART2_TEST), 1);
        assert_eq!(solve_part2(TEST_INPUT), 9);
    }
}
