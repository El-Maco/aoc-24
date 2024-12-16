use crate::utils::load_input;

pub fn run() {
    let input = load_input(9);
    // let part1 = solve_part1(&input);
    // println!("part1: {part1}");
    let part2 = solve_part2(&input);
    println!("part2: {part2}");
}

fn create_id_format(input: &str) -> Vec<Option<u64>> {
    let mut out: Vec<Option<u64>> = vec![];
    let mut id = 0;
    input.trim().chars().enumerate().for_each(|(i, ch)| {
        let count: u32 = ch.to_digit(10).unwrap();
        (0..count).for_each(|_|{
            match i % 2 {
                0 =>  out.push(Some(id)),
                1 =>  out.push(None),
                _ => { eprint!("No arm to handle id {id}")}
            }
        });
        if i % 2 == 0 {
            id += 1;
        }
    });
    out
}


fn compress_disk(disk: &mut Vec<Option<u64>>) {
    let free_space_count: usize = disk.iter().filter(|ch| ch.is_none()).collect::<Vec<_>>().len();
    loop {
        let free_space_index = disk.iter().position(|ch| ch.is_none()).unwrap_or(disk.len());
        let last_digit_rev = disk.iter().rev().position(|ch| ch.is_some()).unwrap_or(0);
        let last_digit_index = disk.len() - 1 - last_digit_rev;
        if free_space_index >= disk.len() - free_space_count {
            break;
        }
        println!("Swapping {free_space_index} with {last_digit_index}");
        disk.swap(free_space_index, last_digit_index);


    }
}

fn checksum(compressed_disk: &Vec<Option<u64>>) -> u128 {
    compressed_disk.iter().enumerate().map(|(i, &ch)| {
        match ch {
            None => 0,
            Some(digit) => {
                let res = i as u128 * digit as u128;
                res
            }
        }
    }).sum()
}

fn solve_part1(input: &str) -> u128 {
    let mut id_formatted = create_id_format(input);
    compress_disk(&mut id_formatted);
    let tot = checksum(&id_formatted);
    tot
}

fn compress_disk_mv_file(disk: &mut Vec<Option<u64>>) {
    let mut curr_id = disk.iter().max().unwrap().unwrap();

    loop {
        let id_index = disk.len() - 1 - disk.iter().rev().position(|v| if let Some(id) = v { *id == curr_id } else { false }).unwrap_or(0);
        let id_count = disk.iter().filter(|v| if let Some(id) = v { *id == curr_id } else { false }).count();
        println!("{curr_id} index: {id_index} count: {id_count}");

        // find free space for it
        match disk.clone().windows(id_count).enumerate().find(|(_, w)| w.iter().all(|v| v.is_none())) {
            Some((i, free_space)) => {
                free_space.iter().enumerate().for_each(|(j, _)| { if i + j < id_index - j { disk.swap(i + j, id_index - j); } });
            },
            None => {}
        }
        if curr_id == 0 { break; }
        curr_id -= 1;
    }

}

fn vec_to_string(disk: &Vec<Option<u64>>) -> String {
    let mut s = String::new();
    disk.iter().for_each(|v| if let Some(value) = v { s.push_str(&value.to_string()) } else { s.push('.') });
    s
}

fn string_to_vec(input: &str) -> Vec<Option<u64>> {
        input.chars().map(|ch| match ch {
            '.' => None,
            _ => Some(ch.to_digit(10).unwrap() as u64)
        }).collect::<Vec<Option<u64>>>()
}

fn solve_part2(input: &str) -> u128 {
    let mut id_formatted = create_id_format(input);
    compress_disk_mv_file(&mut id_formatted);
    let tot = checksum(&id_formatted);
    tot
}

#[cfg(test)]
mod tests {

    static TEST_INPUT: &str = "2333133121414131402";
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_create_id_format() {
        assert_eq!(create_id_format(TEST_INPUT), string_to_vec("00...111...2...333.44.5555.6666.777.888899"));
    }

    #[test]
    fn test_compress_disk() {
        let mut input = string_to_vec("00...111...2...333.44.5555.6666.777.888899");
        let result = string_to_vec("0099811188827773336446555566..............");
        compress_disk(&mut input);
        assert_eq!(input, result);
    }

    #[test]
    fn test_compress_disk2() {
        let mut input = string_to_vec("00...111...2...333.44.5555.6666.777.888899");
        let result = string_to_vec("00992111777.44.333....5555.6666.....8888..");
        compress_disk_mv_file(&mut input);
        assert_eq!(input, result);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 2858);
    }

}
