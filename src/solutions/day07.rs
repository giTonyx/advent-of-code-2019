use crate::intcode::{read_input, IntCode};
use crate::solver::Solver;
use permutator::Permutation;
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
        let mut max_thrust = 0i64;

        let mut phases = [0, 1, 2, 3, 4];
        phases.permutation().for_each(|p| {
            let thrust = run_async_chain(input, &p);
            if thrust > max_thrust {
                max_thrust = thrust;
            }
        });
        max_thrust
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut max_thrust = 0i64;

        let mut phases = [5, 6, 7, 8, 9];
        phases.permutation().for_each(|p| {
            let thrust = run_async_chain(input, &p);
            if thrust > max_thrust {
                max_thrust = thrust;
            }
        });
        max_thrust
    }
}

fn run_async_chain(program: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let mut int_code_a = IntCode::new(program);
    int_code_a.input.push(phases[0]);
    int_code_a.input.push(0);

    let mut int_code_b = IntCode::new(program);
    int_code_b.input.push(phases[1]);

    let mut int_code_c = IntCode::new(program);
    int_code_c.input.push(phases[2]);

    let mut int_code_d = IntCode::new(program);
    int_code_d.input.push(phases[3]);

    let mut int_code_e = IntCode::new(program);
    int_code_e.input.push(phases[4]);

    while !int_code_e.finished {
        int_code_a.advance(&mut int_code_b.input);
        int_code_b.advance(&mut int_code_c.input);
        int_code_c.advance(&mut int_code_d.input);
        int_code_d.advance(&mut int_code_e.input);
        int_code_e.advance(&mut int_code_a.input);
    }
    int_code_e.last_output
}

#[test]
fn test_run_aync_chain() {
    let program: Vec<i64> = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let phase: Vec<i64> = vec![9, 7, 8, 5, 6];
    let output = run_async_chain(&program, &phase);
    println!("{}", output);
    assert!(output == 18216);

    let program: Vec<i64> = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let phase: Vec<i64> = vec![9, 8, 7, 5, 6];
    let output = run_async_chain(&program, &phase);
    println!("{}", output);
    assert!(output == 138547328); // number on web page seems wrong
}
