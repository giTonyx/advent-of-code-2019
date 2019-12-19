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
        count_tractor(input, 50, 50)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        find_starting_point(input, 100, 100)
    }
}

fn find_starting_point(program: &Vec<i64>, width: i64, height: i64) -> u64 {
    let mut beam_map = BeamMap::new();

    let maximum_distance = width * height;
    // The starting number has been obtained by looking at the map, but could be obtained programmatically
    // with a binary search
    for total_distance in 1740..maximum_distance {
        for x in 0..total_distance {
            for y in 0..total_distance {
                if x + y != total_distance {
                    continue;
                }
                if verify(program, &mut beam_map, x, y, width, height) {
                    return (x * 10000 + y) as u64;
                }
            }
        }
    }
    0
}

struct BeamMap {
    points: HashMap<Coord, bool>,
}

impl BeamMap {
    pub fn new() -> BeamMap {
        BeamMap {
            points: HashMap::new(),
        }
    }
    fn get(&mut self, program: &Vec<i64>, x: i64, y: i64) -> bool {
        let coord = Coord { x: x, y: y };
        if self.points.contains_key(&coord) {
            return *self.points.get(&coord).unwrap();
        }
        let mut output = IntInput::new();
        let mut intcode = IntCode::new(program);
        intcode.input.push(x);
        intcode.input.push(y);
        intcode.advance(&mut output);
        let result = output.get() == 1;
        self.points.insert(coord, result);
        result
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

fn verify(
    program: &Vec<i64>,
    beam_map: &mut BeamMap,
    startx: i64,
    starty: i64,
    width: i64,
    height: i64,
) -> bool {
    for y in starty..starty + height {
        for x in startx..startx + width {
            let result = beam_map.get(program, x, y);
            if !result {
                return false;
            }
        }
    }
    true
}

fn count_tractor(program: &Vec<i64>, width: usize, height: usize) -> u64 {
    let mut affected = 0;

    for y in 0..height {
        for x in 0..width {
            let mut intcode = IntCode::new(program);
            let mut output = IntInput::new();
            intcode.input.push(x as i64);
            intcode.input.push(y as i64);
            intcode.advance(&mut output);
            let result = output.get();
            if result == 1 {
                affected += 1;
            //print!("#");
            } else {
                //print!(".");
            }
        }
        //println!("");
    }
    affected
}
