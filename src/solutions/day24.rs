use crate::coords::{Coord, Direction};
use crate::solver::Solver;
use core::fmt;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = u32;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut buffer = String::new();
        BufReader::new(r).read_to_string(&mut buffer).expect("");
        buffer
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut diversities = HashSet::new();
        let mut life = Life::from_string(input);
        diversities.insert(life.biodiversity);
        loop {
            life = life.step();
            if diversities.contains(&life.biodiversity) {
                break;
            }
            diversities.insert(life.biodiversity);
        }
        life.biodiversity
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut life = MultiLife::from_string(input);
        for _ in 0..200 {
            life.step();
        }
        life.num_bugs
    }
}

struct MultiLife {
    cells: HashMap<MultiCoord, bool>,
    num_bugs: u64,
}

impl MultiLife {
    pub fn from_string(s: &String) -> MultiLife {
        let mut cells = HashMap::new();
        let mut num_bugs = 0;

        let mut y = 0;
        for line in s.trim().to_string().split('\n') {
            let mut x = 0;
            for char in line.chars() {
                if !(x == 2 && y == 2) {
                    let coord = MultiCoord {
                        level: 0,
                        x: x,
                        y: y,
                    };
                    match char {
                        '#' => {
                            cells.insert(coord, true);
                            num_bugs += 1;
                        }
                        '.' => {
                            cells.insert(coord, false);
                        }
                        _ => unimplemented!(),
                    }
                }
                x += 1;
            }
            y += 1;
        }

        MultiLife {
            cells: cells,
            num_bugs: num_bugs,
        }
    }

    fn step(&mut self) {
        let min_level = self.cells.keys().map(|c| c.level).min().unwrap();
        let max_level = self.cells.keys().map(|c| c.level).max().unwrap();
        self.add_level(min_level - 1);
        self.add_level(max_level + 1);

        let mut new_cells = HashMap::new();

        for (c, value) in self.cells.iter() {
            let nearbies = self.sum_nearby(c);
            match *value {
                true => {
                    if nearbies != 1 {
                        new_cells.insert(c.clone(), false);
                        self.num_bugs -= 1;
                    } else {
                        new_cells.insert(c.clone(), true);
                    }
                }
                false => {
                    if nearbies == 1 || nearbies == 2 {
                        new_cells.insert(c.clone(), true);
                        self.num_bugs += 1;
                    } else {
                        new_cells.insert(c.clone(), false);
                    }
                }
            }
        }
        self.cells = new_cells;
    }

    fn sum_nearby(&self, coord: &MultiCoord) -> u8 {
        coord
            .nearby()
            .iter()
            .map(|c| self.cells.get(c).unwrap_or(&false))
            .map(|b| if *b { 1 } else { 0 })
            .sum()
    }

    fn add_level(&mut self, level: i32) {
        for x in 0..5 {
            for y in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }
                self.cells.insert(
                    MultiCoord {
                        level: level,
                        x: x,
                        y: y,
                    },
                    false,
                );
            }
        }
    }
}

#[test]
fn test_multi_step_count() {
    let s = "....#\n\
             #..#.\n\
             #.?##\n\
             ..#..\n\
             #...."
        .to_string();
    let mut life = MultiLife::from_string(&s);
    for _ in 0..10 {
        life.step();
    }
    assert!(life.num_bugs == 99);
}

#[test]
fn test_multi_sum_nearby() {
    let s = ".....\n\
             .....\n\
             .....\n\
             #....\n\
             .#..."
        .to_string();
    let life = MultiLife::from_string(&s);
    assert!(
        life.sum_nearby(&MultiCoord {
            level: 0,
            x: 0,
            y: 3
        }) == 0
    );
    assert!(
        life.sum_nearby(&MultiCoord {
            level: 0,
            x: 0,
            y: 2
        }) == 1
    );
    assert!(
        life.sum_nearby(&MultiCoord {
            level: 0,
            x: 1,
            y: 3
        }) == 2
    );
}

#[test]
fn test_multi_from_string() {
    let s = ".....\n\
             .....\n\
             .....\n\
             #....\n\
             .#..."
        .to_string();
    let life = MultiLife::from_string(&s);
    assert!(life.cells.len() == 24);
}

impl fmt::Debug for MultiLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        let min_level = self.cells.keys().map(|c| c.level).min().unwrap();
        let max_level = self.cells.keys().map(|c| c.level).max().unwrap();

        for level in min_level..=max_level {
            buffer.push_str(format!("Level: {}\n", level).as_str());
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        buffer.push('?');
                        continue;
                    }
                    if *self
                        .cells
                        .get(&MultiCoord {
                            level: level,
                            x: x,
                            y: y,
                        })
                        .unwrap()
                    {
                        buffer.push('#')
                    } else {
                        buffer.push('.')
                    };
                }
                buffer.push('\n');
            }
            buffer.push('\n');
        }

        write!(f, "{}", buffer)
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct MultiCoord {
    level: i32,
    x: u8,
    y: u8,
}

impl fmt::Debug for MultiCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L:{} X:{} Y:{}", self.level, self.x, self.y)
    }
}

impl MultiCoord {
    fn nearby(&self) -> Vec<MultiCoord> {
        let mut cells = Vec::new();

        // On same level
        if self.x > 0 && (self.x != 3 || self.y != 2) {
            cells.push(MultiCoord {
                level: self.level,
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x < 4 && (self.x != 1 || self.y != 2) {
            cells.push(MultiCoord {
                level: self.level,
                x: self.x + 1,
                y: self.y,
            });
        }

        if self.y > 0 && (self.x != 2 || self.y != 3) {
            cells.push(MultiCoord {
                level: self.level,
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y < 4 && (self.x != 2 || self.y != 1) {
            cells.push(MultiCoord {
                level: self.level,
                x: self.x,
                y: self.y + 1,
            });
        }

        // Outer Level
        if self.x == 0 {
            cells.push(MultiCoord {
                level: self.level - 1,
                x: 1,
                y: 2,
            });
        }
        if self.x == 4 {
            cells.push(MultiCoord {
                level: self.level - 1,
                x: 3,
                y: 2,
            });
        }
        if self.y == 0 {
            cells.push(MultiCoord {
                level: self.level - 1,
                x: 2,
                y: 1,
            });
        }
        if self.y == 4 {
            cells.push(MultiCoord {
                level: self.level - 1,
                x: 2,
                y: 3,
            });
        }

        // Inner Level
        if self.x == 1 && self.y == 2 {
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 1,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 2,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 3,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 4,
            });
        }

        if self.x == 3 && self.y == 2 {
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 1,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 2,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 3,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 4,
            });
        }

        if self.x == 2 && self.y == 1 {
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 1,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 2,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 3,
                y: 0,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 0,
            });
        }

        if self.x == 2 && self.y == 3 {
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 0,
                y: 4,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 1,
                y: 4,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 2,
                y: 4,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 3,
                y: 4,
            });
            cells.push(MultiCoord {
                level: self.level + 1,
                x: 4,
                y: 4,
            });
        }

        cells
    }
}

#[test]
fn test_multi_nearby() {
    assert!(
        MultiCoord {
            level: 0,
            x: 3,
            y: 3
        }
        .nearby()
        .len()
            == 4
    );
    assert!(
        MultiCoord {
            level: 1,
            x: 1,
            y: 1
        }
        .nearby()
        .len()
            == 4
    );
    assert!(
        MultiCoord {
            level: 1,
            x: 3,
            y: 0
        }
        .nearby()
        .len()
            == 4
    );
    assert!(
        MultiCoord {
            level: 1,
            x: 4,
            y: 0
        }
        .nearby()
        .len()
            == 4
    );
    assert!(
        MultiCoord {
            level: 0,
            x: 3,
            y: 2
        }
        .nearby()
        .len()
            == 8
    );
    assert!(
        MultiCoord {
            level: 1,
            x: 3,
            y: 2
        }
        .nearby()
        .len()
            == 8
    );
}

struct Life {
    biodiversity: u32,
}

impl Life {
    pub fn from_string(s: &String) -> Life {
        let mut biodiversity = 0;
        let mut current_index = 0;

        for line in s.trim().to_string().split('\n') {
            for char in line.chars() {
                match char {
                    '#' => {
                        biodiversity += 2u32.pow(current_index);
                    }
                    _ => (),
                }
                current_index += 1;
            }
        }

        Life {
            biodiversity: biodiversity,
        }
    }

    fn get_bit(&self, coord: Coord) -> u32 {
        if coord.x < 0 || coord.x > 4 || coord.y < 0 || coord.y > 4 {
            return 0;
        }
        let idx = (coord.y * 5 + coord.x) as u32;
        (self.biodiversity >> (idx)) % 2
    }

    fn nearby_sum(&self, coord: Coord) -> u32 {
        Direction::all()
            .iter()
            .map(|d| self.get_bit(coord.next(&d)))
            .sum()
    }

    fn step(&self) -> Life {
        let mut new_biodiversity = 0;
        for x in 0..5 {
            for y in 0..5 {
                let coord = Coord { x: x, y: y };
                let idx = (y * 5 + x) as u32;
                let nearby = self.nearby_sum(coord);
                match self.get_bit(coord) {
                    0 => {
                        if nearby == 1 || nearby == 2 {
                            new_biodiversity += 2u32.pow(idx);
                        }
                    }
                    1 => {
                        if nearby == 1 {
                            new_biodiversity += 2u32.pow(idx);
                        }
                    }
                    _ => unimplemented!(),
                }
            }
        }

        Life {
            biodiversity: new_biodiversity,
        }
    }
}

#[test]
fn test_step() {
    let s1 = "....#\n\
              #..#.\n\
              #..##\n\
              ..#..\n\
              #...."
        .to_string();
    let l1 = Life::from_string(&s1);

    let s2 = "#..#.\n\
              ####.\n\
              ###.#\n\
              ##.##\n\
              .##.."
        .to_string();
    let l2 = Life::from_string(&s2);

    let s3 = "#####\n\
              ....#\n\
              ....#\n\
              ...#.\n\
              #.###"
        .to_string();
    let l3 = Life::from_string(&s3);

    assert!(l1.step().biodiversity == l2.biodiversity);
    assert!(l2.step().biodiversity == l3.biodiversity);
}

#[test]
fn test_nearby_sum() {
    let s = ".....\n\
             .....\n\
             .....\n\
             #....\n\
             .#..."
        .to_string();
    let life = Life::from_string(&s);
    assert!(life.nearby_sum(Coord { x: 0, y: 3 }) == 0);
    assert!(life.nearby_sum(Coord { x: 0, y: 2 }) == 1);
    assert!(life.nearby_sum(Coord { x: 1, y: 3 }) == 2);
}

#[test]
fn test_get_bit() {
    let s = ".....\n\
             .....\n\
             .....\n\
             #....\n\
             .#..."
        .to_string();
    let life = Life::from_string(&s);
    assert!(life.get_bit(Coord { x: 0, y: 0 }) == 0);
    assert!(life.get_bit(Coord { x: 1, y: 0 }) == 0);
    assert!(life.get_bit(Coord { x: 0, y: 1 }) == 0);
    assert!(life.get_bit(Coord { x: 0, y: 3 }) == 1);
    assert!(life.get_bit(Coord { x: 1, y: 3 }) == 0);
    assert!(life.get_bit(Coord { x: 1, y: 4 }) == 1);
}

#[test]
fn test_from_string() {
    let s = ".....\n\
             .....\n\
             .....\n\
             #....\n\
             .#..."
        .to_string();
    assert!(Life::from_string(&s).biodiversity == 2129920);
}
