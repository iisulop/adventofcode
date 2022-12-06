use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Result};

use common::get_filename;

type Rucksack = (HashSet<char>, HashSet<char>);
type Rucksacks = Vec<Rucksack>;

trait Priority {
    fn priority(&self) -> Result<usize>;
}

impl Priority for char {
    fn priority(&self) -> Result<usize> {
        match (self.is_ascii_uppercase(), self.is_ascii_lowercase()) {
            (false, true) => Ok(*self as usize - 97 + 1),
            (true, false) => Ok(*self as usize - 65 + 27),
            (_, _) => Err(anyhow!("Unexpected item type")),
        }
    }
}

fn parse_line(line: Result<String, std::io::Error>) -> Result<Rucksack> {
    let line = line?;
    Ok((
        line.chars().take(line.len() / 2).collect(),
        line.chars().skip(line.len() / 2).collect(),
    ))
}

fn read_file(filename: &str) -> Result<Rucksacks> {
    let file = File::open(filename)?;
    let rucksacks = BufReader::new(&file)
        .lines()
        .filter(|line| line.as_ref().map(|l| !l.is_empty()).unwrap_or(true))
        .map(parse_line)
        .collect::<Result<Rucksacks>>()?;
    Ok(rucksacks)
}

fn calculate_priorities_part1(rucksacks: &Rucksacks) -> Result<usize> {
    Ok(rucksacks
        .iter()
        .flat_map(|sack| sack.0.intersection(&sack.1).map(|el| el.priority()))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum())
}

fn calculate_priorities_part2(rucksacks: &Rucksacks) -> Result<usize> {
    let res = rucksacks
        .iter()
        .map(|(a, b)| a.union(b).collect::<HashSet<&char>>())
        .collect::<Vec<HashSet<&char>>>();
    let res = res
        .chunks(3)
        .map(|chunk| {
            let [a, b, c]: &[HashSet<&char>; 3] = chunk.try_into()?;
            let bc = b.intersection(c).cloned().collect::<HashSet<&char>>();
            Ok(a.intersection(&bc).cloned().collect::<HashSet<&char>>())
        })
        .collect::<Result<Vec<HashSet<&char>>>>()?;
    let res = res
        .iter()
        .map(|rucksack_badge| {
            Ok(rucksack_badge
                .iter()
                .map(|badge| badge.priority())
                .collect::<Result<Vec<usize>>>()?
                .iter()
                .sum::<usize>())
        })
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum();
    Ok(res)
}

fn main() -> Result<()> {
    let filename = get_filename();
    let results = read_file(&filename)?;
    let priorities1 = calculate_priorities_part1(&results)?;
    println!("points part 1: {}", priorities1);
    let priorities2 = calculate_priorities_part2(&results)?;
    println!("points part 2: {}", priorities2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{calculate_priorities_part1, calculate_priorities_part2, read_file};

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        assert_eq!(calculate_priorities_part1(&input).unwrap(), 157);
    }

    #[test]
    fn test_part2() {
        let input = read_file("input/example").unwrap();
        assert_eq!(calculate_priorities_part2(&input).unwrap(), 70);
    }
}
