use crate::intcode::{read_input, IntCode, IntInput};
use crate::solver::Solver;
use std::collections::{HashMap, HashSet, VecDeque};
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
        let mut intcode = IntCode::new(input);
        let mut repair_drone = RepairBot::new();
        repair_drone.explore(&mut intcode);
        repair_drone.find_oxygen_distance()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut intcode = IntCode::new(input);
        let mut repair_drone = RepairBot::new();
        repair_drone.explore(&mut intcode);
        repair_drone.diffuse()
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
    pub fn value(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
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
                y: self.y + 1,
            },
            Direction::South => Coord {
                x: self.x,
                y: self.y - 1,
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
struct Path {
    steps: Vec<Direction>,
}

impl Path {
    pub fn new() -> Path {
        Path { steps: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

struct RepairBot {
    position: Coord,
    oxygen: Option<Coord>,
    cells: HashMap<Coord, Cell>,
}

impl RepairBot {
    pub fn new() -> RepairBot {
        let origin = Coord { x: 0, y: 0 };
        let mut cells = HashMap::new();
        cells.insert(origin.clone(), Cell::EMPTY);
        RepairBot {
            position: origin,
            oxygen: None,
            cells: cells,
        }
    }

    fn is_a_wall(&self, cell: &Coord) -> bool {
        if self.cells.contains_key(cell) && *self.cells.get(cell).unwrap() == Cell::WALL {
            return true;
        }
        false
    }

    fn is_empty_cell(&self, cell: &Coord) -> bool {
        if self.cells.contains_key(cell) && *self.cells.get(cell).unwrap() == Cell::EMPTY {
            return true;
        }
        false
    }

    fn empty_cell_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|(_, v)| **v == Cell::EMPTY)
            .count()
    }

    fn oxygen_cells(&self) -> Vec<Coord> {
        self.cells
            .iter()
            .filter(|(_, v)| **v == Cell::OXYGEN)
            .map(|(k, _)| *k)
            .collect::<Vec<Coord>>()
    }

    fn diffuse(&mut self) -> u64 {
        let mut turn = 0;

        while self.empty_cell_count() > 0 {
            turn += 1;
            let oxygen_cells = self.oxygen_cells();
            for cell in oxygen_cells {
                for direction in Direction::all() {
                    let next_cell = cell.next(&direction);
                    if self.is_empty_cell(&next_cell) {
                        self.cells.insert(next_cell, Cell::OXYGEN);
                    }
                }
            }
        }

        turn
    }
    fn explore(&mut self, intcode: &mut IntCode) {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(Coord { x: 0, y: 0 });
        for d in Direction::all() {
            to_visit.push_back(self.position.next(&d));
        }

        while !to_visit.is_empty() {
            let cell_to_visit = to_visit.pop_back().unwrap();

            // In case we found already it's a wall
            if self.is_a_wall(&cell_to_visit) {
                continue;
            }

            self.move_to(cell_to_visit, intcode);

            visited.insert(self.position);
            for d in Direction::all() {
                let nearby_cell = self.position.next(&d);
                if self.is_a_wall(&nearby_cell) {
                    continue;
                }
                if !visited.contains(&nearby_cell) {
                    to_visit.push_back(nearby_cell);
                }
            }

            if *self.cells.get(&self.position).unwrap() == Cell::OXYGEN {
                self.oxygen = Some(self.position);
            }
        }
    }

    fn find_oxygen_distance(&mut self) -> u64 {
        self.minimum_distance(&self.oxygen.unwrap(), &Coord { x: 0, y: 0 })
    }

    fn move_to(&mut self, target: Coord, intcode: &mut IntCode) {
        let mut output = IntInput::new();
        for direction in self.plot_move_to(target).steps {
            intcode.input.push(direction.value());
            intcode.advance(&mut output);
            let next_cell = self.position.next(&direction);
            match output.get() {
                0 => {
                    self.cells.insert(next_cell, Cell::WALL);
                }
                1 => {
                    self.position = next_cell;
                    self.cells.insert(next_cell, Cell::EMPTY);
                }
                2 => {
                    self.position = next_cell;
                    self.cells.insert(next_cell, Cell::OXYGEN);
                }
                r => println!("ERROR: Unexpcted answer: {}", r),
            }
        }
    }

    fn plot_move_to(&self, target: Coord) -> Path {
        self.find_best_path(&self.position, &target)
    }

    fn minimum_distance(&self, source: &Coord, destination: &Coord) -> u64 {
        self.find_best_path(source, destination).len() as u64
    }

    fn update_path(paths: &mut HashMap<Coord, Path>, cell: &Coord, path: &Path) -> bool {
        if paths.contains_key(cell) {
            if paths.get(cell).unwrap().len() > path.len() {
                paths.insert(*cell, path.clone());
                return true;
            }
        } else {
            paths.insert(*cell, path.clone());
            return true;
        }
        false
    }

    fn find_best_path(&self, source: &Coord, destination: &Coord) -> Path {
        let mut queue = VecDeque::new();
        queue.push_back((*source, Path::new()));

        let mut paths = HashMap::new();

        while !queue.is_empty() {
            let (cell, path_so_far) = queue.pop_front().unwrap();

            // Check if we are at destination
            if cell == *destination {
                Self::update_path(&mut paths, destination, &path_so_far);
            } else {
                // Only the destination can be out of the map
                if !self.cells.contains_key(&cell) {
                    continue;
                }
                // Cannot pass through a wall
                if *self.cells.get(&cell).unwrap() == Cell::WALL {
                    continue;
                }
                if !Self::update_path(&mut paths, &cell, &path_so_far) {
                    continue;
                }

                for direction in Direction::all() {
                    let next_cell = cell.next(&direction);
                    let mut next_path = path_so_far.clone();
                    next_path.steps.push(direction.clone());
                    queue.push_back((next_cell, next_path));
                }
            }
        }

        paths.get(destination).unwrap().clone()
    }
}

#[test]
fn test_best_path() {
    let mut repair_bot = RepairBot::new();
    repair_bot.cells.insert(Coord { x: 1, y: 0 }, Cell::WALL);
    repair_bot.cells.insert(Coord { x: 0, y: 1 }, Cell::EMPTY);
    repair_bot.cells.insert(Coord { x: 1, y: 1 }, Cell::EMPTY);
    repair_bot.cells.insert(Coord { x: 2, y: 1 }, Cell::EMPTY);
    repair_bot.cells.insert(Coord { x: 2, y: 0 }, Cell::EMPTY);
    assert!(repair_bot.minimum_distance(&Coord { x: 0, y: 0 }, &Coord { x: 0, y: 1 }) == 1);
    assert!(repair_bot.minimum_distance(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 0 }) == 1);
    assert!(repair_bot.minimum_distance(&Coord { x: 0, y: 0 }, &Coord { x: 2, y: 0 }) == 4);
}

#[derive(PartialEq)]
enum Cell {
    EMPTY,
    WALL,
    OXYGEN,
}
