
use crate::utils::{load_input, parse_input};

pub fn run() {
    let input = load_input(10);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

type Trail = Vec<MapPosition>;

struct MapPosition {
    x: i32,
    y: i32,
    height: Option<u8>,
}

fn find_trails(x: usize, y: usize, rows: &Vec<Vec<u8>>, trails: &mut Vec<Trail>) {
    let height = rows[x][y];
    let position: MapPosition = MapPosition {x: x as i32, y: y as i32, height: Some()};

    let Some(new_pos) =
}

fn solve_part1(input: &str) -> u32 {
    let row = parse_input(input);
    let map: Vec<Vec<u8>> = row.iter().map(|row| row.chars().map(|ch| ch.to_digit(10).expect("Could not parse digit") as u8).collect()).collect();

    let mut trails: Vec<Trail> = vec![];
    map.iter().enumerate().for_each(|(j, row)| {
        row.iter().enumerate().for_each(|(i, d)| {
            if *d == 0 {
                let mut found_trails: Vec<Trail> = vec![];
                find_trails(i, j, &map, &mut found_trails);
                if found_trails.len() > 0 {
                    trails.append(&mut found_trails);
                }

            }
        });
    });


    0
}

fn solve_part2(input: &str) -> u32 {
    todo![]
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
        assert_eq!(solve_part2(TEST_INPUT), todo!());
    }

}
