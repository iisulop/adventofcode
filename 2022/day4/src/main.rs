use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

use common::get_filename;

//type Sections = (HashSet<usize>, HashSet<usize>);

#[derive(Debug)]
struct SectionSet {
    start: usize,
    end: usize,
}

impl FromStr for SectionSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ends = s.split('-').collect::<Vec<&str>>();
        let (start, end): (&str, &str) = (
            ends.get(0)
                .ok_or_else(|| anyhow!("Could not get start {}", s))?,
            ends.get(1)
                .ok_or_else(|| anyhow!("Could not get end {}", s))?,
        );
        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

impl SectionSet {
    fn fully_overlaps(&self, other: &Self) -> bool {
        if self.start >= other.start && self.end <= other.end {
            true
        } else {
            false
        }
    }

    fn partially_overlaps(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.start 
            || self.start <= other.end && self.end >= other.end {
            true
        } else {
            false
        }
    }
}

type Sections = (SectionSet, SectionSet);

fn get_owned_element<T>(coll: &mut Vec<T>, idx: usize) -> Option<T> {
    if let Some(_) = coll.get(idx) {
        Some(coll.swap_remove(idx))
    } else {
        None
    }
}

fn read_file(filename: &str) -> Result<Vec<Sections>> {
    let file = File::open(filename)?;
    let res = BufReader::new(&file)
        .lines()
        .filter(|el| if let Ok(x) = el { !x.is_empty() } else { false })
        .map(|line| {
            let mut sections = line?
                .split(',')
                .map(|section_range| SectionSet::from_str(section_range))
                .collect::<Result<Vec<SectionSet>>>()?;
            Ok((
                get_owned_element(&mut sections, 0)
                    .ok_or_else(|| anyhow!("Could not get first section {:#?}", sections))?,
                get_owned_element(&mut sections, 0)
                    .ok_or_else(|| anyhow!("Could not get second section {:#?}", sections))?,
            ))
        })
        .collect::<Result<Vec<Sections>>>()?;
    Ok(res)
}

fn calculate_fully_overlapping(assignments: &Vec<Sections>) -> usize {
    assignments.iter().fold(0, |num, (el1, el2)| {
        if el1.fully_overlaps(&el2) || el2.fully_overlaps(&el1) {
            num + 1
        } else {
            num
        }
    })
}

fn calculate_partially_overlapping(assignments: &Vec<Sections>) -> usize {
    assignments.iter().fold(0, |num, (el1, el2)| {
        if el1.partially_overlaps(&el2) || el2.partially_overlaps(&el1) {
            num + 1
        } else {
            num
        }
    })
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let assignments = read_file(&filename)?;
    let overlapping = calculate_fully_overlapping(&assignments);
    println!("overlapping part 1: {}", overlapping);
    let overlapping = calculate_partially_overlapping(&assignments);
    println!("overlapping part 2: {}", overlapping);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{read_file, calculate_fully_overlapping, calculate_partially_overlapping};

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        assert_eq!(calculate_fully_overlapping(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_file("input/example").unwrap();
        assert_eq!(calculate_partially_overlapping(&input), 4);
    }
}
