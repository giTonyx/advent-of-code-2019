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
    let mut program_input = input.clone();
    program_input[1] = noun;
    program_input[2] = verb;
    run_intcode(&mut program_input);
    program_input[0]
}

fn run_intcode(input: &mut Vec<i64>) {
    let mut program_counter = 0;
    loop {
        match input[program_counter] {
            1 => {
                let store_idx = input[program_counter + 3] as usize;
                let value1_idx = input[program_counter + 1] as usize;
                let value2_idx = input[program_counter + 2] as usize;
                input[store_idx] = input[value1_idx] + input[value2_idx];
            }
            2 => {
                let store_idx = input[program_counter + 3] as usize;
                let value1_idx = input[program_counter + 1] as usize;
                let value2_idx = input[program_counter + 2] as usize;
                input[store_idx] = input[value1_idx] * input[value2_idx];
            }
            99 => {
                break;
            }
            _ => (println!("Unexpected OPCODE")),
        }
        program_counter += 4;
    }
}
