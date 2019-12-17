use crate::intcode::{read_input, IntCode, IntInput};
use crate::solver::Solver;
use core::fmt;
use std::collections::HashMap;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut intcode = IntCode::new(input);
        let camera = Camera::create(&mut intcode);
        camera.aligment_sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut intcode = IntCode::new(input);
        let camera = Camera::create(&mut intcode);
        let (routine, a, b, c) = camera.draw_path();

        let mut intcode = IntCode::new(input);
        camera.send_path(&mut intcode, routine, a, b, c)
    }
}

struct Camera {
    cells: HashMap<Coord, Cell>,
    robot: Coord,
}

impl fmt::Debug for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        let max_x = self.cells.iter().map(|(k, _)| k.x).max().unwrap();
        let max_y = self.cells.iter().map(|(k, _)| k.y).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x: x, y: y };
                if coord == self.robot {
                    buffer.push('^');
                } else {
                    if *self.cells.get(&coord).unwrap() == Cell::SCAFFOLD {
                        buffer.push('#');
                    } else {
                        buffer.push('.');
                    }
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

impl Camera {
    fn send_path(
        &self,
        intcode: &mut IntCode,
        routine: String,
        a: String,
        b: String,
        c: String,
    ) -> i64 {
        intcode.memory.store(0, 2);
        for c in routine.chars() {
            intcode.input.push(c as i64);
        }
        intcode.input.push(10);
        for c in a.chars() {
            intcode.input.push(c as i64);
        }
        intcode.input.push(10);
        for c in b.chars() {
            intcode.input.push(c as i64);
        }
        intcode.input.push(10);
        for c in c.chars() {
            intcode.input.push(c as i64);
        }
        intcode.input.push(10);
        intcode.input.push('n' as i64);
        intcode.input.push(10);

        let mut output = IntInput::new();
        intcode.advance(&mut output);

        let mut return_code = 0;
        while output.has_input() {
            return_code = output.get();
        }
        return_code
    }

    fn draw_path(&self) -> (String, String, String, String) {
        let mut position = self.robot.clone();
        let mut direction = Direction::North;

        let mut buffer = String::new();

        loop {
            let mut consecutive = 0;
            loop {
                let forward_position = position.next(&direction);
                if self.is_scaffolding(&forward_position) {
                    position = forward_position;
                    consecutive += 1;
                } else {
                    if consecutive > 0 {
                        // Add to Instructions
                        buffer += consecutive.to_string().as_str();
                        buffer.push(',');
                    }
                    break;
                }
            }
            let left_position = position.next(&direction.left());
            if self.is_scaffolding(&left_position) {
                // Add L instruction
                buffer.push('L');
                buffer.push(',');
                direction = direction.left();
                continue;
            }
            let right_position = position.next(&direction.right());
            if self.is_scaffolding(&right_position) {
                // Add R instruction
                buffer.push('R');
                buffer.push(',');
                direction = direction.right();
                continue;
            }
            break;
        }
        let (a, b, c) = split_string(&buffer);
        let composition = compose(&buffer, &a, &b, &c);
        (composition, a, b, c)
    }

    fn is_scaffolding(&self, coord: &Coord) -> bool {
        if !self.cells.contains_key(coord) {
            return false;
        }
        *self.cells.get(coord).unwrap() == Cell::SCAFFOLD
    }

    pub fn create(intcode: &mut IntCode) -> Camera {
        let mut cells = HashMap::new();
        let mut output = IntInput::new();
        intcode.advance(&mut output);

        let mut x = 0;
        let mut y = 0;

        let mut robot_x = 0;
        let mut robot_y = 0;

        while output.has_input() {
            let view = output.get();
            match view {
                35 => {
                    cells.insert(Coord { x: x, y: y }, Cell::SCAFFOLD);
                    x += 1;
                }
                94 => {
                    robot_x = x;
                    robot_y = y;
                    cells.insert(Coord { x: x, y: y }, Cell::SCAFFOLD);
                    x += 1;
                }
                46 => {
                    cells.insert(Coord { x: x, y: y }, Cell::EMPTY);
                    x += 1
                }
                10 => {
                    x = 0;
                    y += 1;
                }
                _ => println!("ERROR: Unexpected output: {}", view),
            }
        }
        Camera {
            cells: cells,
            robot: Coord {
                x: robot_x,
                y: robot_y,
            },
        }
    }

    fn is_intersection(&self, coord: &Coord) -> bool {
        if *self.cells.get(coord).unwrap() != Cell::SCAFFOLD {
            return false;
        }
        for d in Direction::all() {
            let next_coord = coord.next(&d);
            if !self.cells.contains_key(&next_coord) {
                return false;
            }
            if *self.cells.get(&next_coord).unwrap() != Cell::SCAFFOLD {
                return false;
            }
        }
        true
    }

    fn aligment_sum(&self) -> u64 {
        self.cells
            .iter()
            .filter(|(k, _)| self.is_intersection(k))
            .map(|(k, _)| (k.x * k.y) as u64)
            .sum()
    }
}

fn split_string(data: &String) -> (String, String, String) {
    let parts = data
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for al in 1..11 {
        for bl in 1..11 {
            for cl in 1..11 {
                for skip1 in 0..30 {
                    for skip2 in 0..30 {
                        if (al + bl + skip1 + skip2 + cl + 1) > parts.len() {
                            continue;
                        }
                        let a_start = 0;
                        let b_start = al + skip1;
                        let c_start = b_start + bl + skip2;

                        let a = vector_to_string(&parts, a_start, al);
                        let b = vector_to_string(&parts, b_start, bl);
                        let c = vector_to_string(&parts, c_start, cl);

                        if a.len() > 20 {
                            continue;
                        }
                        if b.len() > 20 {
                            continue;
                        }
                        if c.len() > 20 {
                            continue;
                        }
                        if can_compose(data, &a, &b, &c) {
                            return (a, b, c);
                        }
                    }
                }
            }
        }
    }
    (String::new(), String::new(), String::new()) // should never happen
}

fn vector_to_string(vec: &Vec<String>, start: usize, len: usize) -> String {
    vec[start..start + len].join(",")
}

#[test]
fn test_split_string() {
    split_string(&"L,10,R,10,R,5,L,10,R,10,L,1,".to_string());
}

fn compose(data: &String, a: &String, b: &String, c: &String) -> String {
    let mut composition = String::new();

    let mut position = 0;
    loop {
        if position == data.len() {
            break;
        }
        if position + a.len() < data.len() && &data[position..position + a.len()] == a.as_str() {
            position += a.len() + 1;
            composition.push('A');
            composition.push(',');
            continue;
        };
        if position + b.len() < data.len() && &data[position..position + b.len()] == b.as_str() {
            position += b.len() + 1;
            composition.push('B');
            composition.push(',');
            continue;
        };
        if position + c.len() < data.len() && &data[position..position + c.len()] == c.as_str() {
            position += c.len() + 1;
            composition.push('C');
            composition.push(',');
            continue;
        };
        break;
    }
    composition[..composition.len() - 1].to_string()
}

fn can_compose(data: &String, a: &String, b: &String, c: &String) -> bool {
    let mut position = 0;
    loop {
        if position == data.len() {
            break;
        }
        if position + a.len() < data.len() && &data[position..position + a.len()] == a.as_str() {
            position += a.len() + 1;
            continue;
        };
        if position + b.len() < data.len() && &data[position..position + b.len()] == b.as_str() {
            position += b.len() + 1;
            continue;
        };
        if position + c.len() < data.len() && &data[position..position + c.len()] == c.as_str() {
            position += c.len() + 1;
            continue;
        };
        break;
    }
    position == data.len()
}

#[test]
fn test_can_compose() {
    assert!(
        can_compose(
            &"A,B,A,B,C,D,E,A,C,C,D,E,".to_string(),
            &"A,B".to_string(),
            &"A,C".to_string(),
            &"C,D,E".to_string(),
        ) == true
    );
    assert!(
        can_compose(
            &"A,B,A,B,C,A,E,A,C,C,D,E,".to_string(),
            &"A,B".to_string(),
            &"A,C".to_string(),
            &"C,D,E".to_string(),
        ) == false
    );
}

#[derive(PartialEq)]
enum Cell {
    EMPTY,
    SCAFFOLD,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn next(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::North => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
    }
}
