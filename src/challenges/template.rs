use crate::utils::load_input;

pub fn run() {
    let input = load_input(3);
    let part1 = solve_part1(&input);
    println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn solve_part1(input: &str) -> u32 {
    todo![]
}

fn solve_part2(input: &str) -> u32 {
    todo![]
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "";
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), todo!());
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), todo!());
    }

}
