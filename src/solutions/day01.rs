use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        unimplemented!()
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        unimplemented!()
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        unimplemented!()
    }
}
