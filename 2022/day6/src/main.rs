use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Error, Result};

use common::{get_filename, Part};

fn read_file(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut buf = String::new();
    BufReader::new(&file).read_line(&mut buf)?;
    Ok(buf)
}

fn get_start_pos(input: &str, part: Part) -> Option<usize> {
    let num_consecutive_letters = match part {
        Part::Part1 => 4,
        Part::Part2 => 14,
    };
    input
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(num_consecutive_letters)
        .enumerate()
        .fold(None, |res: Option::<usize>, (idx, chars)| {
            if res.is_none() {
                if chars.iter().collect::<HashSet<&char>>().len() == num_consecutive_letters {
                    // println!("yes: {:?}", chars);
                    Some(num_consecutive_letters + idx)
                } else {
                    // println!("not: {:?}", chars);
                    None
                }
            } else {
                res
            }
        })
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let input = read_file(&filename)?;
    let start_pos_part1 =
        get_start_pos(&input, Part::Part1).ok_or_else(|| anyhow!("Failed to find start position"))?;
    println!("part 1: {}", start_pos_part1);
    let start_pos_part2 =
        get_start_pos(&input, Part::Part2).ok_or_else(|| anyhow!("Failed to find start position"))?;
    println!("part 2: {}", start_pos_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_start_pos, read_file};
    use common::Part;

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        let start_pos = get_start_pos(&input, Part::Part1).unwrap();
        assert_eq!(start_pos, 7)
    }

    #[test]
    fn test_part2() {
        let input = read_file("input/example").unwrap();
        let start_pos = get_start_pos(&input, Part::Part2).unwrap();
        assert_eq!(start_pos, 19)
    }
}
