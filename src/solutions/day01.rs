use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        count_increases(input)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let sums = input
            .windows(3)
            .map(|window| window.iter().sum())
            .collect::<Vec<_>>();

        count_increases(&sums)
    }
}

fn count_increases(input: &[i64]) -> i64 {
    input.windows(2).fold(
        0,
        |acc, window| {
            if window[1] > window[0] {
                acc + 1
            } else {
                acc
            }
        },
    )
}
