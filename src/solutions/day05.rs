use crate::intcode::{IntCode, IntInput};
use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        let r = BufReader::new(r);
        r.split(b',')
            .flatten()
            .filter_map(|v| String::from_utf8(v).ok())
            .filter_map(|v| v.to_owned().trim().to_string().parse().ok())
            .collect()
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        let mut intcode = IntCode::new(input);
        let mut output = IntInput::new();
        intcode.input.push(1i64);
        intcode.advance(&mut output);
        let mut last_output = 0;
        while output.has_input() {
            last_output = output.get();
        }
        last_output
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        let mut intcode = IntCode::new(input);
        let mut output = IntInput::new();
        intcode.input.push(5i64);
        intcode.advance(&mut output);
        let mut last_output = 0;
        while output.has_input() {
            last_output = output.get();
        }
        last_output
    }
}
