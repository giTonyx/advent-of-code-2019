use crate::intcode::{read_input, IntCode, IntInput};
use crate::solver::Solver;
use std::collections::HashMap;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut int_code = IntCode::new(input);
        let mut output = IntInput::new();
        let mut block_tiles = 0;
        while !int_code.finished {
            int_code.advance(&mut output);
            while output.has_input() {
                output.get();
                output.get();
                if output.get() == 2 {
                    block_tiles += 1;
                }
            }
        }
        block_tiles
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut int_code = IntCode::new(input);
        let mut game_output = IntInput::new();
        let mut game = ArcadeGame::new();
        int_code.memory.store(0, 2);

        loop {
            int_code.advance(&mut game_output);
            game.process_input(&mut game_output);
            int_code.input.push(game.get_paddle_move());
            if game.screen.block_count() == 0 {
                break;
            }
            if int_code.finished {
                break;
            }
        }
        game.score
    }
}

#[derive(PartialEq)]
enum Cell {
    BLOCK,
    WALL,
}

struct Screen {
    cells: HashMap<(i64, i64), Cell>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            cells: HashMap::new(),
        }
    }

    pub fn block_count(&self) -> usize {
        self.cells.values().filter(|v| **v == Cell::BLOCK).count()
    }

    pub fn set_tile(&mut self, x: i64, y: i64, tile: i64) {
        match tile {
            1 => {
                self.cells.insert((x, y), Cell::WALL);
            }
            2 => {
                self.cells.insert((x, y), Cell::BLOCK);
            }
            _ => (),
        }
    }
}

struct ArcadeGame {
    ball_x: i64,
    ball_y: i64,
    paddle_x: i64,
    paddle_y: i64,
    score: u64,
    screen: Screen,
}

impl ArcadeGame {
    pub fn new() -> ArcadeGame {
        ArcadeGame {
            ball_x: 0,
            ball_y: 0,
            paddle_x: 0,
            paddle_y: 0,
            score: 0,
            screen: Screen::new(),
        }
    }
    pub fn process_input(&mut self, input: &mut IntInput) {
        while input.has_input() {
            let x = input.get();
            let y = input.get();
            let tile = input.get();

            if x == -1 && y == 0 {
                self.score = tile as u64;
                continue;
            }
            match tile {
                3 => {
                    self.paddle_x = x;
                    self.paddle_y = y;
                }
                4 => {
                    self.ball_x = x;
                    self.ball_y = y;
                }
                _ => self.screen.set_tile(x, y, tile),
            }
        }
    }
    pub fn get_paddle_move(&self) -> i64 {
        if self.paddle_x < self.ball_x {
            return 1;
        }
        if self.paddle_x > self.ball_x {
            return -1;
        }
        0
    }
}
