use crate::solver::Solver;
use permutator::Permutation;
use std::io;
use std::io::{BufRead, BufReader};

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

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut max_thrust = 0i64;

        let mut phases = [0, 1, 2, 3, 4];
        phases.permutation().for_each(|p| {
            let thrust = run_chain(input, &p);
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

fn run_chain(program: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let mut input_a = Vec::new();
    input_a.push(phases[0]);
    input_a.push(0);

    let mut input_b = Vec::new();
    input_b.push(phases[1]);
    input_b.push(run_intcode(&mut (program.clone()), &input_a));

    let mut input_c = Vec::new();
    input_c.push(phases[2]);
    input_c.push(run_intcode(&mut (program.clone()), &input_b));

    let mut input_d = Vec::new();
    input_d.push(phases[3]);
    input_d.push(run_intcode(&mut (program.clone()), &input_c));

    let mut input_e = Vec::new();
    input_e.push(phases[4]);
    input_e.push(run_intcode(&mut (program.clone()), &input_d));

    run_intcode(&mut (program.clone()), &input_e)
}

fn run_async_chain(program: &Vec<i64>, phases: &Vec<i64>) -> i64 {
    let mut input_a = IntInput::new();
    input_a.push(phases[0]);
    input_a.push(0);
    let mut code_a = IntCode::new(program, &mut input_a);

    let mut input_b = IntInput::new();
    input_b.push(phases[1]);
    let mut code_b = IntCode::new(program, &mut input_b);

    let mut input_c = IntInput::new();
    input_c.push(phases[2]);
    let mut code_c = IntCode::new(program, &mut input_c);

    let mut input_d = IntInput::new();
    input_d.push(phases[3]);
    let mut code_d = IntCode::new(program, &mut input_d);

    let mut input_e = IntInput::new();
    input_e.push(phases[4]);
    let mut code_e = IntCode::new(program, &mut input_e);

    while !code_e.finished {
        code_a.advance(code_b.input);
        code_b.advance(code_c.input);
        code_c.advance(code_d.input);
        code_d.advance(code_e.input);
        code_e.advance(code_a.input);
    }
    code_e.last_output
}

pub struct IntInput {
    data: Vec<i64>,
    counter: usize,
}

impl IntInput {
    pub fn new() -> IntInput {
        IntInput {
            data: Vec::new(),
            counter: 0,
        }
    }
    pub fn has_input(&self) -> bool {
        self.counter < self.data.len()
    }
    pub fn push(&mut self, value: i64) {
        self.data.push(value);
    }
    pub fn get(&mut self) -> i64 {
        self.counter += 1;
        self.data[self.counter - 1]
    }
}

pub struct IntCode<'a> {
    program: Vec<i64>,
    input: &'a mut IntInput,
    program_counter: usize,
    last_output: i64,
    finished: bool,
}

impl<'a> IntCode<'a> {
    pub fn new(program: &Vec<i64>, input: &'a mut IntInput) -> IntCode<'a> {
        IntCode {
            program: program.clone(),
            input: input,
            program_counter: 0,
            last_output: 0,
            finished: false,
        }
    }

    pub fn advance(&mut self, output: &mut IntInput) {
        if self.finished {
            println!("Computer called after having finished!");
            return;
        }
        loop {
            let opcode = self.program[self.program_counter] % 100;
            let parameters = self.program[self.program_counter] / 100;
            match opcode {
                1 => {
                    let store_idx = self.program[self.program_counter + 3] as usize;
                    let value1 = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let value2 = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize]
                    } else {
                        self.program[self.program_counter + 2]
                    };
                    self.program[store_idx] = value1 + value2;
                    self.program_counter += 4;
                }
                2 => {
                    let store_idx = self.program[self.program_counter + 3] as usize;
                    let value1 = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let value2 = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize]
                    } else {
                        self.program[self.program_counter + 2]
                    };
                    self.program[store_idx] = value1 * value2;
                    self.program_counter += 4;
                }
                3 => {
                    if !self.input.has_input() {
                        return;
                    }
                    let value = self.input.get();
                    let store_idx = self.program[self.program_counter + 1] as usize;
                    self.program[store_idx] = value;
                    self.program_counter += 2;
                }
                4 => {
                    let output_value = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    self.last_output = output_value;
                    output.push(self.last_output);
                    self.program_counter += 2;
                }
                5 => {
                    let test_value = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let jump_location = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize] as usize
                    } else {
                        self.program[self.program_counter + 2] as usize
                    };
                    if test_value == 0 {
                        self.program_counter += 3;
                    } else {
                        self.program_counter = jump_location;
                    }
                }
                6 => {
                    let test_value = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let jump_location = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize] as usize
                    } else {
                        self.program[self.program_counter + 2] as usize
                    };
                    if test_value == 0 {
                        self.program_counter = jump_location;
                    } else {
                        self.program_counter += 3;
                    }
                }
                7 => {
                    let store_idx = self.program[self.program_counter + 3] as usize;
                    let value1 = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let value2 = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize]
                    } else {
                        self.program[self.program_counter + 2]
                    };
                    self.program[store_idx] = if value1 < value2 { 1 } else { 0 };
                    self.program_counter += 4;
                }
                8 => {
                    let store_idx = self.program[self.program_counter + 3] as usize;
                    let value1 = if (parameters % 10) == 0 {
                        self.program[self.program[self.program_counter + 1] as usize]
                    } else {
                        self.program[self.program_counter + 1]
                    };
                    let value2 = if ((parameters / 10) % 10) == 0 {
                        self.program[self.program[self.program_counter + 2] as usize]
                    } else {
                        self.program[self.program_counter + 2]
                    };
                    self.program[store_idx] = if value1 == value2 { 1 } else { 0 };
                    self.program_counter += 4;
                }

                99 => {
                    self.finished = true;
                    break;
                }
                _ => (println!("Unexpected OPCODE")),
            }
        }
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
