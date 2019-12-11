use crate::solutions::day11::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::solver::Solver;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        BufReader::new(r)
            .split(b',')
            .flatten()
            .filter_map(|v| String::from_utf8(v).ok())
            .filter_map(|v| v.to_owned().trim().to_string().parse().ok())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut int_code = IntCode::new(input);
        let mut robot = PaintRobot::new();
        paint(&mut int_code, &mut robot);
        robot.painted_cells
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut int_code = IntCode::new(input);
        let mut robot = PaintRobot::new();
        robot.paint(1);
        paint(&mut int_code, &mut robot);
        robot.draw();
        "JFBERBUH".to_string() // from visual representation
    }
}

fn paint(int_code: &mut IntCode, robot: &mut PaintRobot) {
    let mut output = IntInput::new();
    while !int_code.finished {
        int_code.input.push(robot.read() as i64);
        int_code.advance(&mut output);
        robot.paint(output.get() as u8);
        match output.get() {
            0 => robot.rotate_left(),
            1 => robot.rotate_right(),
            _ => println!("ERROR: Unexpected command"),
        }
        robot.advance();
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn rotate_left(&self) -> Direction {
        match self {
            UP => LEFT,
            LEFT => DOWN,
            DOWN => RIGHT,
            RIGHT => UP,
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            UP => RIGHT,
            LEFT => UP,
            DOWN => LEFT,
            RIGHT => DOWN,
        }
    }
}
struct PaintRobot {
    x: i64,
    y: i64,
    direction: Direction,
    painted_cells: u64,
    cells: HashMap<(i64, i64), u8>,
}

impl PaintRobot {
    pub fn new() -> PaintRobot {
        PaintRobot {
            x: 0,
            y: 0,
            direction: Direction::UP,
            painted_cells: 0,
            cells: HashMap::new(),
        }
    }

    pub fn rotate_left(&mut self) {
        self.direction = self.direction.rotate_left();
    }

    pub fn rotate_right(&mut self) {
        self.direction = self.direction.rotate_right();
    }

    pub fn advance(&mut self) {
        match self.direction {
            UP => {
                self.y += 1;
            }
            DOWN => {
                self.y -= 1;
            }
            LEFT => {
                self.x -= 1;
            }
            RIGHT => {
                self.x += 1;
            }
        }
    }
    pub fn paint(&mut self, color: u8) {
        if !self.cells.contains_key(&(self.x, self.y)) {
            self.painted_cells += 1;
        }
        self.cells.insert((self.x, self.y), color);
    }

    fn read_at(&self, x: i64, y: i64) -> u8 {
        if self.cells.contains_key(&(x, y)) {
            self.cells.get(&(x, y)).unwrap().clone()
        } else {
            0u8
        }
    }

    pub fn read(&self) -> u8 {
        self.read_at(self.x, self.y)
    }

    pub fn draw(&self) {
        let min_x = self.cells.keys().map(|k| k.0).min().unwrap();
        let max_x = self.cells.keys().map(|k| k.0).max().unwrap();
        let min_y = self.cells.keys().map(|k| k.1).min().unwrap();
        let max_y = self.cells.keys().map(|k| k.1).max().unwrap();

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                match self.read_at(x, y) {
                    1 => print!("#"),
                    _ => print!("."),
                }
            }
            println!("");
        }
    }
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
    memory: Memory,
    input: IntInput,
    program_counter: i64,
    last_output: i64,
    finished: bool,
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
