use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::{Match, Regex};

use common::{get_filename, Part};

const MOVE_RE: &str = r"move (?P<num>\d+) from (?P<from>\d+) to (?P<to>\d+)";

#[derive(Clone, Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}
type Stacks = Vec<Vec<char>>;
type Moves = Vec<Move>;
type Input = (Stacks, Moves);

fn usize_from_match(m: Option<Match>) -> Result<usize> {
    m.ok_or_else(|| anyhow!("cannot read to"))?
        .as_str()
        .parse::<usize>()
        .map_err(|e| anyhow!("Cannot parse number: {}", e))
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(MOVE_RE).unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            Ok(Move {
                num: usize_from_match(captures.name("num"))?,
                from: usize_from_match(captures.name("from"))?,
                to: usize_from_match(captures.name("to"))?,
            })
        } else {
            Err(anyhow!("Could not parse move: {}", s))
        }
    }
}

fn read_file(filename: &str) -> Result<Input> {
    let file = File::open(filename)?;
    let mut line_iter = BufReader::new(&file).lines();
    //.filter(|el| if let Ok(x) = el { !x.is_empty() } else { false })
    let stacks: Stacks = line_iter
        .by_ref()
        .take_while(|line| {
            if let Ok(l) = line {
                l.contains('[')
            } else {
                false
            }
        })
        .fold(Ok(Stacks::new()), |mut columns: Result<Stacks>, line| {
            let line = line?.chars().collect::<Vec<char>>().chunks(4).fold(
                Ok::<Vec<Option<char>>, anyhow::Error>(Vec::<Option<char>>::new()),
                |mut row, chunk| {
                    let c = chunk
                        .get(1)
                        .ok_or_else(|| anyhow!("Could not get crate contents"))?
                        .to_owned();
                    if c != ' ' {
                        row.as_mut()
                            .map(|r| r.push(Some(c)))
                            .map_err(|e| anyhow!("{}", e))?;
                    } else {
                        row.as_mut()
                            .map(|r| r.push(None))
                            .map_err(|e| anyhow!("{}", e))?;
                    }
                    row
                },
            )?;
            if let Ok(ref mut cols) = columns {
                cols.resize(line.len(), Vec::new());
            }
            for (idx, maybe_crate) in line.into_iter().enumerate() {
                if let Some(crate_) = maybe_crate {
                    if let Ok(ref mut cols) = columns {
                        cols.get_mut(idx).unwrap().push(crate_);
                    }
                }
            }
            columns
        })?;
    let stacks = stacks
        .into_iter()
        .map(|s| s.into_iter().rev().collect())
        .collect();
    let moves = line_iter
        .filter(|line| line.as_ref().map(|l| !l.is_empty()).unwrap_or(false))
        .map(|el| Move::from_str(&el?))
        .collect::<Result<Moves>>()?;
    Ok((stacks, moves))
}

fn move_crates(input: Input, part: Part) -> Result<Stacks> {
    input
        .1
        .iter()
        .fold(Ok(input.0), |mut stacks: Result<Stacks>, mov| {
            if let Ok(ref mut s) = stacks {
                let mut crates: Vec<char> = {
                    let origin_stack = s
                        .get_mut(mov.from - 1)
                        .ok_or_else(|| anyhow!("Could not get origin stack"))?;
                    let stack = origin_stack
                        .drain((origin_stack.len() - mov.num)..);
                    let stack = if part == Part::Part1 {
                        stack.rev().collect()
                    } else { stack.collect() };
                    stack
                };
                let target_stack = s
                    .get_mut(mov.to - 1)
                    .ok_or_else(|| anyhow!("Could not get target stack"))?;
                target_stack.append(&mut crates)
            }
            //println!("stacks: {:#?}", stacks);
            stacks
        })
}

fn get_top_crates(stacks: &Stacks) -> String {
    stacks.iter().filter_map(|s| s.last()).collect()
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let input = read_file(&filename)?;
    let top_crates_part1 = get_top_crates(&move_crates(input.clone(), Part::Part1)?);
    println!("part 1: {}", top_crates_part1);
    let top_crates_part2 = get_top_crates(&move_crates(input, Part::Part2)?);
    println!("part 2: {}", top_crates_part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_top_crates, move_crates, read_file};

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        let top_crates = get_top_crates(&move_crates(input).unwrap());
        assert_eq!(top_crates, "CMZ")
    }
}
