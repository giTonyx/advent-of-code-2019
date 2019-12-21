use crate::solver::Solver;
use std::io;
use crate::intcode::{read_input, IntCode, IntInput};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut intcode = IntCode::new(input);
        let mut output = IntInput::new();
        send_string(&mut intcode, "OR A J".to_string());
        send_string(&mut intcode, "AND C J".to_string());
        send_string(&mut intcode, "NOT J J".to_string());
        send_string(&mut intcode, "AND D J".to_string());
        send_string(&mut intcode, "WALK".to_string());
        intcode.advance(&mut output);
        let mut last_code = 0;
        while output.has_input() {
            last_code = output.get();
        }
        last_code
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut intcode = IntCode::new(input);
        let mut output = IntInput::new();
        send_string(&mut intcode, "OR A J".to_string());
        send_string(&mut intcode, "AND B J".to_string());
        send_string(&mut intcode, "AND C J".to_string());
        send_string(&mut intcode, "NOT J J".to_string());
        send_string(&mut intcode, "AND D J".to_string());
        send_string(&mut intcode, "OR E T".to_string());
        send_string(&mut intcode, "OR H T".to_string());
        send_string(&mut intcode, "AND T J".to_string());
        send_string(&mut intcode, "RUN".to_string());
        intcode.advance(&mut output);
        let mut last_code = 0;
        while output.has_input() {
            last_code = output.get();
        }
        last_code
    }
}

fn send_string(intcode: &mut IntCode, data: String ) {
    for c in data.chars()  {
        intcode.input.push(c as i64);
    }
    intcode.input.push(10);
}
