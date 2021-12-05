use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<i32>>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Seek + io::Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines()
            .flatten()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        _ => panic!("invalid char: {}", c),
                    })
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let freqs = input
            .iter()
            .fold(None, |acc: Option<Vec<i32>>, vec| {
                if let Some(acc) = acc {
                    let mut res = vec![0; acc.len()];
                    for ((r, a), b) in res.iter_mut().zip(acc).zip(vec) {
                        *r = a + b;
                    }
                    Some(res)
                } else {
                    Some(vec.clone())
                }
            })
            .unwrap();

        let gamma_rate_str = freqs
            .iter()
            .map(|v| {
                if v > &((input.len() / 2) as i32) {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>();
        let epsilon_rate_str = freqs
            .iter()
            .map(|v| {
                if v > &((input.len() / 2) as i32) {
                    '0'
                } else {
                    '1'
                }
            })
            .collect::<String>();

        let gamma_rate = i64::from_str_radix(&gamma_rate_str, 2).unwrap();
        let epsilon_rate = i64::from_str_radix(&epsilon_rate_str, 2).unwrap();

        gamma_rate * epsilon_rate
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}
