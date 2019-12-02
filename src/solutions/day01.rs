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
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        input.iter().map(|&mass| mass / 3 - 2).sum()
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        input
            .iter()
            .map(|&mass| {
                let mut fuel = 0;
                let mut module_mass = mass;
                loop {
                    module_mass = module_mass / 3 - 2;
                    if module_mass > 0 {
                        fuel += module_mass;
                    } else {
                        break;
                    }
                }
                fuel
            })
            .sum()
    }
}
