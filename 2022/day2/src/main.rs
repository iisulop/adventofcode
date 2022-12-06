use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{anyhow, Error};

use common::get_filename;

type Games = Vec<(Shape, Shape)>;

#[derive(Debug)]
enum RpsResult {
    Victory,
    Draw,
    Loss,
}

impl FromStr for RpsResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RpsResult::Loss),
            "Y" => Ok(RpsResult::Draw),
            "Z" => Ok(RpsResult::Victory),
            x => Err(anyhow!("Unexpected result: {}", x)),
        }
    }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            x => Err(anyhow!("Unexpected pick for RPS: {}", x)),
        }
    }
}

impl Shape {
    fn shape_points(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn victory_points(&self, theirs: &Shape) -> usize {
        match (self, theirs) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 6,
            (Shape::Scissors, Shape::Scissors) => 3,
        }
    }
    fn points(&self, theirs: &Shape) -> usize {
        self.shape_points() + self.victory_points(theirs)
    }

    fn shape_from_expected_result(&self, expected_result: RpsResult) -> Shape {
        match (self, expected_result) {
            (Shape::Rock, RpsResult::Victory) => Shape::Paper,
            (Shape::Rock, RpsResult::Draw) => Shape::Rock,
            (Shape::Rock, RpsResult::Loss) => Shape::Scissors,
            (Shape::Paper, RpsResult::Victory) => Shape::Scissors,
            (Shape::Paper, RpsResult::Draw) => Shape::Paper,
            (Shape::Paper, RpsResult::Loss) => Shape::Rock,
            (Shape::Scissors, RpsResult::Victory) => Shape::Rock,
            (Shape::Scissors, RpsResult::Draw) => Shape::Scissors,
            (Shape::Scissors, RpsResult::Loss) => Shape::Paper,
        }
    }
}

fn read_file_part1(filename: &str) -> Result<Games, Error> {
    let file = File::open(filename)?;
    let mut games = Games::new();
    for line in BufReader::new(&file).lines() {
        let line = line?;
        if !line.is_empty() {
            let picks: Vec<&str> = line.split(' ').map(|c| c.trim()).collect();
            games.push((
                Shape::from_str(
                    picks
                        .first()
                        .ok_or_else(|| anyhow!("Could not parse theirs"))?,
                )?,
                Shape::from_str(
                    picks
                        .get(1)
                        .ok_or_else(|| anyhow!("Could not parse ours"))?,
                )?,
            ));
        }
    }
    Ok(games)
}

fn read_file_part2(filename: &str) -> Result<Games, Error> {
    let file = File::open(filename)?;
    let mut games = Games::new();
    for line in BufReader::new(&file).lines() {
        let line = line?;
        if !line.is_empty() {
            let picks: Vec<&str> = line.split(' ').map(|c| c.trim()).collect();
            let theirs = Shape::from_str(
                picks
                    .first()
                    .ok_or_else(|| anyhow!("Could not parse theirs"))?,
            )?;
            let ours = theirs.shape_from_expected_result(RpsResult::from_str(
                picks
                    .get(1)
                    .ok_or_else(|| anyhow!("Could not parse ours"))?,
            )?);
            games.push((theirs, ours));
        }
    }
    Ok(games)
}

fn calculate_points(results: Games) -> usize {
    results.iter().fold(0, |coll, el| coll + el.1.points(&el.0))
}

fn main() -> Result<(), Error> {
    let filename = get_filename();
    let results1 = read_file_part1(&filename)?;
    let points1 = calculate_points(results1);
    println!("points part 1: {}", points1);

    let results2 = read_file_part2(&filename)?;
    let points2 = calculate_points(results2);
    println!("points part 2: {}", points2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{calculate_points, read_file_part1, read_file_part2};

    #[test]
    fn test_part1() {
        let input = read_file_part1("input/example").unwrap();
        assert_eq!(calculate_points(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = read_file_part2("input/example").unwrap();
        assert_eq!(calculate_points(input), 12);
    }
}
