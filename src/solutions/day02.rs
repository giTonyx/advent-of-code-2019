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
            .filter_map(|v| v.parse().ok())
            .collect()
    }

    fn solve_first(&self, input: &Vec<i64>) -> i64 {
        run_program(input, 12, 2)
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        for noun in 0..=99 {
            for verb in 0..=99 {
                if run_program(input, noun, verb) == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        0 // Should never happen
    }
}

fn run_program(input: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut intcode = IntCode::new(input);
    intcode.memory.store(1, noun);
    intcode.memory.store(2, verb);
    let mut output = IntInput::new();
    intcode.advance(&mut output);
    intcode.memory.read(0)
}
