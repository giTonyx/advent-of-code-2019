use crate::intcode::{read_input, IntCode, IntInput};
use crate::solver::Solver;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut int_code = IntCode::new(input);
        int_code.input.push(1);
        let mut output = IntInput::new();
        while !int_code.finished {
            int_code.advance(&mut output);
        }
        int_code.last_output
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut int_code = IntCode::new(input);
        int_code.input.push(2);
        let mut output = IntInput::new();
        while !int_code.finished {
            int_code.advance(&mut output);
        }
        int_code.last_output
    }
}
