// GENERATED ON BUILD, DO NOT EDIT
use crate::solver::Solver;

mod day01;
mod day02;
mod day03;

pub fn exec_day(day: i32) {
    match day {
        1 => day01::Problem {}.solve(day),
        2 => day02::Problem {}.solve(day),
        3 => day03::Problem {}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
