// DO NOT EDIT THIS FILE - Last generated: 2019-01-08 16:02:50.477133 UTC
use crate::solver::Solver;

mod day01;


pub fn exec_day(day: i32) {
    match day {
        1 => day01::Problem {}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
