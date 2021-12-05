use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub enum Direction {
    Up,
    Down,
    Forward,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("invalid direction {}", s),
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(Direction, i64)>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Seek + io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|l| {
                let split = l.split(' ').collect::<Vec<_>>();
                (Direction::from(split[0]), split[1].parse().unwrap())
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut pos = 0;
        let mut depth = 0;

        input.iter().for_each(|(dir, amount)| match dir {
            Direction::Up => depth -= amount,
            Direction::Down => depth += amount,
            Direction::Forward => pos += amount,
        });

        pos * depth
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut aim = 0;
        let mut pos = 0;
        let mut depth = 0;

        input.iter().for_each(|(dir, amount)| match dir {
            Direction::Up => aim -= amount,
            Direction::Down => aim += amount,
            Direction::Forward => {
                pos += amount;
                depth += aim * amount;
            }
        });

        pos * depth
    }
}
