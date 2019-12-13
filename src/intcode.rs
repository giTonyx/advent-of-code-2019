use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};

pub fn read_input<R: io::Read>(r: R) -> Vec<i64> {
    BufReader::new(r)
        .split(b',')
        .flatten()
        .filter_map(|v| String::from_utf8(v).ok())
        .filter_map(|v| v.to_owned().trim().to_string().parse().ok())
        .collect()
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
pub struct Memory {
    mem: HashMap<i64, i64>,
}

impl Memory {
    pub fn new(program: &Vec<i64>) -> Memory {
        let mut memory_map = HashMap::new();
        for i in 0..program.len() {
            memory_map.insert(i as i64, program[i]);
        }
        Memory { mem: memory_map }
    }

    pub fn store(&mut self, address: i64, value: i64) {
        if address < 0 {
            println!("ERROR: Negative address!");
        }
        *self.mem.entry(address).or_insert(0) = value;
    }

    pub fn read(&self, address: i64) -> i64 {
        if address < 0 {
            println!("ERROR: Negative address!");
        }
        self.mem.get(&address).unwrap_or(&0i64).clone()
    }
}

pub struct IntCode {
    pub memory: Memory,
    pub input: IntInput,
    program_counter: i64,
    pub last_output: i64,
    pub finished: bool,
    relative_base: i64,
}

impl IntCode {
    pub fn new(program: &Vec<i64>) -> IntCode {
        IntCode {
            memory: Memory::new(program),
            input: IntInput::new(),
            program_counter: 0,
            last_output: 0,
            finished: false,
            relative_base: 0,
        }
    }

    fn get_specific_parameter(offset: i64, parameters: i64) -> i64 {
        let mut params = parameters;
        for _ in 1..offset {
            params = params / 10
        }
        params % 10
    }

    fn get_value(&self, offset: i64, parameters: i64) -> i64 {
        let parameter = IntCode::get_specific_parameter(offset, parameters);

        self.memory.read(match parameter {
            // position
            0 => self.memory.read(self.program_counter + offset),
            // direct
            1 => self.program_counter + offset,
            // relative
            2 => self.memory.read(self.program_counter + offset) + self.relative_base,
            _ => {
                println!("ERROR: Unexpected Parameter type!");
                0i64
            }
        })
    }

    fn get_store_index(&self, offset: i64, parameters: i64) -> i64 {
        let parameter = IntCode::get_specific_parameter(offset, parameters);
        match parameter {
            // position
            0 => self.memory.read(self.program_counter + offset),
            // relative
            2 => self.memory.read(self.program_counter + offset) + self.relative_base,
            _ => {
                println!("ERROR: Unexpected Store parameter");
                0
            }
        }
    }

    pub fn advance(&mut self, output: &mut IntInput) {
        if self.finished {
            println!("Computer called after having finished!");
            return;
        }
        loop {
            let opcode = self.memory.read(self.program_counter) % 100;
            let parameters = self.memory.read(self.program_counter) / 100;
            match opcode {
                1 => {
                    let store_idx = self.get_store_index(3, parameters);
                    let value1 = self.get_value(1, parameters);
                    let value2 = self.get_value(2, parameters);
                    self.memory.store(store_idx, value1 + value2);
                    self.program_counter += 4;
                }
                2 => {
                    let store_idx = self.get_store_index(3, parameters);
                    let value1 = self.get_value(1, parameters);
                    let value2 = self.get_value(2, parameters);
                    self.memory.store(store_idx, value1 * value2);
                    self.program_counter += 4;
                }
                3 => {
                    if !self.input.has_input() {
                        return;
                    }
                    let value = self.input.get();
                    let store_idx = self.get_store_index(1, parameters);
                    self.memory.store(store_idx, value);
                    self.program_counter += 2;
                }
                4 => {
                    let output_value = self.get_value(1, parameters);
                    self.last_output = output_value;
                    output.push(self.last_output);
                    self.program_counter += 2;
                }
                5 => {
                    let test_value = self.get_value(1, parameters);
                    let jump_location = self.get_value(2, parameters);
                    if test_value == 0 {
                        self.program_counter += 3;
                    } else {
                        self.program_counter = jump_location;
                    }
                }
                6 => {
                    let test_value = self.get_value(1, parameters);
                    let jump_location = self.get_value(2, parameters);
                    if test_value == 0 {
                        self.program_counter = jump_location;
                    } else {
                        self.program_counter += 3;
                    }
                }
                7 => {
                    let store_idx = self.get_store_index(3, parameters);
                    let value1 = self.get_value(1, parameters);
                    let value2 = self.get_value(2, parameters);
                    self.memory
                        .store(store_idx, if value1 < value2 { 1 } else { 0 });
                    self.program_counter += 4;
                }
                8 => {
                    let store_idx = self.get_store_index(3, parameters);
                    let value1 = self.get_value(1, parameters);
                    let value2 = self.get_value(2, parameters);
                    self.memory
                        .store(store_idx, if value1 == value2 { 1 } else { 0 });
                    self.program_counter += 4;
                }
                9 => {
                    let value = self.get_value(1, parameters);
                    self.relative_base += value;
                    self.program_counter += 2;
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

#[test]
fn test_relative() {
    let mut code_output = IntInput::new();
    let mut code = IntCode::new(&vec![104, 1125899906842624, 99]);
    while !code.finished {
        code.advance(&mut code_output);
    }
    assert!(code.finished);
    assert!(code.last_output == 1125899906842624);

    let mut code_output = IntInput::new();
    let mut code = IntCode::new(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
    while !code.finished {
        code.advance(&mut code_output);
    }
    assert!(code.finished);
    assert!(code.last_output == 1219070632396864);

    let mut code_output = IntInput::new();
    let mut code = IntCode::new(&vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]);
    while !code.finished {
        code.advance(&mut code_output);
    }
    assert!(code.finished);
    assert!(code_output.data.len() == 16);
}
