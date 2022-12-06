use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Error};

use common::get_filename;

type Elves = Vec<Vec<usize>>;

fn read_file(filename: &str) -> Result<Elves, Error> {
    let file = File::open(filename)?;

    let mut elves = Vec::new();
    let mut carrying: Vec<usize> = Vec::new();
    for line in BufReader::new(&file).lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(carrying);
            carrying = Vec::new();
        } else {
            carrying.push(line.trim().parse()?);
        }
    }
    Ok(elves)
}

fn count_carrying(elves: &Elves) -> Vec<usize> {
    elves.iter().map(|elf| elf.iter().sum()).collect()
}

fn get_max_count(elves: &Elves) -> Result<usize, Error> {
    let counts = count_carrying(elves);
    counts.into_iter().max().ok_or_else(|| anyhow!("No counts"))
}

fn get_max_3_sum(elves: &Elves) -> usize {
    let counts = count_carrying(elves);
    counts
        .into_iter()
        .fold(Vec::with_capacity(3), |mut coll: Vec<usize>, el| {
            if coll.len() < 3 {
                coll.push(el);
            } else {
                for old_max in coll.iter_mut() {
                    if el > *old_max {
                        *old_max = el;
                        break;
                    }
                }
            }
            coll.sort();
            coll
        })
        .iter()
        .sum()
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let elves = read_file(&filename)?;
    let max_count = get_max_count(&elves)?;
    let sum = get_max_3_sum(&elves);
    println!("part 1 max: {}", max_count);
    println!("part 2 sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_max_3_sum, get_max_count, read_file};

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        let max_count = get_max_count(&input).unwrap();
        assert_eq!(max_count, 24000)
    }

    #[test]
    fn test_part2() {
        let input = read_file("input/example").unwrap();
        let max_sum = get_max_3_sum(&input);
        assert_eq!(max_sum, 45000)
    }
}
