use crate::intcode::{read_input, IntCode, IntInput};
use crate::solutions::day11::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::solver::Solver;
use std::collections::HashMap;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
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
