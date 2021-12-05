// GENERATED ON BUILD, DO NOT EDIT
use crate::solver::Solver;

mod day01;

pub fn exec_day(day: i32) {
    match day {
        1 => day01::Problem {}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
