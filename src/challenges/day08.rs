use std::collections::{HashMap, HashSet};

use crate::utils::{load_input, parse_input, Position};

pub fn run() {
    let input = load_input(8);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}


fn generate_freq_map(rows: &Vec<&str>) -> HashMap<char, Vec<Position>> {
    let mut freq_map: HashMap<char, Vec<Position>> = HashMap::new();

    rows.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, ch)| match ch {
            '.' => {}
            _ => {
                if let Some(arr) = freq_map.get_mut(&ch) {
                    arr.push(Position::new(x as i32, y as i32));
                } else {
                    freq_map.insert(
                        ch,
                        vec![Position::new(x as i32, y as i32)],
                    );
                }
            }
        });
    });

    freq_map
}

fn print_map(rows: &[&str], antinodes: &HashSet<Position>) {
    rows.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, ch)| {
            if let Some(_) = antinodes.iter().find(|p| p.x == x as i32 && p.y == y as i32) {
                print!("#");
            } else {
                print!("{}", ch);
            }
        });
        print!("\n");
    });
}

fn solve_part1(input: &str) -> u32 {
    let rows = parse_input(input);
    let (width, height) = (rows.first().map_or(0, |row| row.len()), rows.len());
    let freq_map = generate_freq_map(&rows);
    let mut antinode_set: HashSet<Position> = HashSet::new();
    freq_map.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos1| {
            positions.iter().for_each(|pos2| {
                if pos1 == pos2 {
                    return;
                }

                let directional_vec = pos1.diff(&pos2);
                let antinode = pos2.add(&directional_vec);
                if antinode.is_contained(width as i32, height as i32) {
                    antinode_set.insert(antinode);
                }
            });
        })
    });

    print_map(&rows, &antinode_set);

    antinode_set.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    let rows = parse_input(input);
    let (width, height) = (rows.first().map_or(0, |row| row.len()), rows.len());
    let freq_map = generate_freq_map(&rows);
    let mut antinode_set: HashSet<Position> = HashSet::new();
    freq_map.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos1| {
            positions.iter().for_each(|pos2| {
                if pos1 == pos2 {
                    return;
                }

                let directional_vec = pos1.diff(&pos2);
                let mut antinode = pos1.add(&directional_vec);
                while antinode.is_contained(width as i32, height as i32) {
                    antinode_set.insert(antinode);
                    antinode = antinode.add(&directional_vec);
                }
            });
        })
    });

    print_map(&rows, &antinode_set);

    antinode_set.len() as u32

}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 34);
    }

}
