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
        let mut program_input = Vec::new();
        program_input.push(1i64);
        let mut program_code = input.clone();
        run_intcode(&mut program_code, &program_input)
    }

    fn solve_second(&self, input: &Vec<i64>) -> i64 {
        let mut program_input = Vec::new();
        program_input.push(5i64);
        let mut program_code = input.clone();
        run_intcode(&mut program_code, &program_input)
    }
}

fn run_intcode(program: &mut Vec<i64>, input: &Vec<i64>) -> i64 {
    let mut input_counter = 0;
    let mut program_counter = 0;
    let mut last_output = 0i64;
    loop {
        let opcode = program[program_counter] % 100;
        let parameters = program[program_counter] / 100;
        match opcode {
            1 => {
                let store_idx = program[program_counter + 3] as usize;
                let value1 = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let value2 = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize]
                } else {
                    program[program_counter + 2]
                };
                program[store_idx] = value1 + value2;
                program_counter += 4;
            }
            2 => {
                let store_idx = program[program_counter + 3] as usize;
                let value1 = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let value2 = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize]
                } else {
                    program[program_counter + 2]
                };
                program[store_idx] = value1 * value2;
                program_counter += 4;
            }
            3 => {
                let value = input[input_counter];
                input_counter += 1;
                let store_idx = program[program_counter + 1] as usize;
                program[store_idx] = value;
                program_counter += 2;
            }
            4 => {
                let output_value = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                last_output = output_value;
                program_counter += 2;
            }
            5 => {
                let test_value = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let jump_location = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize] as usize
                } else {
                    program[program_counter + 2] as usize
                };
                if test_value == 0 {
                    program_counter += 3;
                } else {
                    program_counter = jump_location;
                }
            }
            6 => {
                let test_value = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let jump_location = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize] as usize
                } else {
                    program[program_counter + 2] as usize
                };
                if test_value == 0 {
                    program_counter = jump_location;
                } else {
                    program_counter += 3;
                }
            }
            7 => {
                let store_idx = program[program_counter + 3] as usize;
                let value1 = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let value2 = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize]
                } else {
                    program[program_counter + 2]
                };
                program[store_idx] = if value1 < value2 { 1 } else { 0 };
                program_counter += 4;
            }
            8 => {
                let store_idx = program[program_counter + 3] as usize;
                let value1 = if (parameters % 10) == 0 {
                    program[program[program_counter + 1] as usize]
                } else {
                    program[program_counter + 1]
                };
                let value2 = if ((parameters / 10) % 10) == 0 {
                    program[program[program_counter + 2] as usize]
                } else {
                    program[program_counter + 2]
                };
                program[store_idx] = if value1 == value2 { 1 } else { 0 };
                program_counter += 4;
            }

            99 => {
                break;
            }
            _ => (println!("Unexpected OPCODE")),
        }
    }
    last_output
}
