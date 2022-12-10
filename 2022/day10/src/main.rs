use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Error, Result};

use common::get_filename;

#[derive(Clone, Debug)]
enum Operation {
    NoOp,
    Addx { delay: isize, value: isize },
}

impl Operation {
    fn new_addx(value: isize) -> Self {
        Self::Addx { delay: 2, value }
    }

    fn get_value(&self) -> isize {
        match self {
            Operation::Addx { value, .. } => *value,
            Operation::NoOp => 0,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<&str>>().as_slice() {
            ["noop"] => Ok(Operation::NoOp),
            ["addx", value] => Ok(Operation::new_addx(value.parse()?)),
            op => Err(anyhow!("Unknown operation {}", op.join(" "))),
        }
    }
}

fn read_file(filename: &str) -> Result<Vec<Operation>> {
    let file = File::open(filename)?;
    let res = BufReader::new(&file)
        .lines()
        .map(|line| Operation::from_str(&line?))
        .collect::<Result<Vec<Operation>>>()?;
    Ok(res)
}

fn calculate_signal_strength(pos: isize, val: isize) -> isize {
    (2 + pos) * val
}

fn sum_of_signal_strengths(operations: &[Operation]) -> isize {
    let strengths = calculate_signal_strengths(operations);
    strengths
        .iter()
        .enumerate()
        .skip(18)
        .step_by(40)
        .fold(0, |sum, (idx, el)| {
            sum + calculate_signal_strength(idx as isize, *el)
        })
}

fn calculate_signal_strengths(operations: &[Operation]) -> Vec<isize> {
    operations
        .iter()
        .fold((Vec::<isize>::new(), 1), |(mut coll, x), el| {
            let new_x = match el {
                Operation::NoOp => {
                    coll.push(x);
                    x
                }
                Operation::Addx { value, .. } => {
                    coll.push(x);
                    let new_x = x + value;
                    coll.push(new_x);
                    new_x
                }
            };
            (coll, new_x)
        })
        .0
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let operations = read_file(&filename)?;
    let result = sum_of_signal_strengths(&operations);
    println!("result part 1: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{read_file, sum_of_signal_strengths};

    #[test]
    fn test_part1() {
        let input = read_file("input/example").unwrap();
        assert_eq!(sum_of_signal_strengths(&input), 13140);
    }
}
