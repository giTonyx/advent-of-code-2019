use crate::solver::Solver;
use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut buffer = String::new();
        BufReader::new(r).read_to_string(&mut buffer).expect("");
        buffer
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let tunnel = TunnelMap::from_string(input);
        tunnel.solve_maze()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let tunnel = TunnelMap::from_string(input);
        tunnel.solve_with_levels()
    }
}

struct TunnelMap {
    cells: HashSet<Coord>,
    portals: HashMap<Coord, Coord>,
    entrance: Coord,
    exit: Coord,
    width: i64,
    height: i64,
}

impl TunnelMap {
    pub fn from_string(data: &String) -> TunnelMap {
        let mut cells = HashSet::new();
        let mut letters = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        let mut y = -1;
        for line in data.split('\n') {
            y += 1;
            max_y = y;
            let mut x = -1;
            for c in line.chars() {
                x += 1;
                if x > max_x {
                    max_x = x
                };
                match c {
                    '#' => (),
                    ' ' => (),
                    '.' => {
                        cells.insert(Coord { x: x, y: y });
                    }
                    'A'..='Z' => {
                        letters.insert(Coord { x: x, y: y }, c);
                    }
                    '\n' => (),
                    '\r' => (),
                    _ => unimplemented!(),
                }
            }
        }

        let mut portals = HashMap::new();
        let mut maybe_entrance = None;
        let mut maybe_exit = None;
        let mut temp_portals = HashMap::new();

        let entrance_name = "AA".to_string();
        let exit_name = "ZZ".to_string();

        for (position, letter) in letters.clone() {
            for dir in Direction::all() {
                let second_position = match dir {
                    Direction::North => position.next(&Direction::South),
                    Direction::West => position.next(&Direction::East),
                    _ => position.next(&dir),
                };

                let tile_position = match dir {
                    Direction::North => position.next(&dir),
                    Direction::West => position.next(&dir),
                    _ => second_position.next(&dir),
                };

                if letters.contains_key(&second_position) && cells.contains(&tile_position) {
                    let second_letter = *letters.get(&second_position).unwrap();
                    let mut portal_name = String::new();
                    portal_name.push(letter);
                    portal_name.push(second_letter);

                    if portal_name == entrance_name {
                        maybe_entrance = Some(tile_position);
                        continue;
                    }
                    if portal_name == exit_name {
                        maybe_exit = Some(tile_position);
                        continue;
                    }

                    if temp_portals.contains_key(&portal_name) {
                        let prev_position = *temp_portals.get(&portal_name).unwrap();
                        portals.insert(prev_position, tile_position);
                        portals.insert(tile_position, prev_position);
                    } else {
                        temp_portals.insert(portal_name, tile_position);
                    }
                    continue;
                }
            }
        }

        TunnelMap {
            cells: cells,
            portals: portals,
            entrance: maybe_entrance.unwrap(),
            exit: maybe_exit.unwrap(),
            width: max_x,
            height: max_y,
        }
    }

    fn nearby_cells(&self, position: Coord) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        for d in Direction::all() {
            let next_position = position.next(&d);
            if self.cells.contains(&next_position) {
                neighbours.push(next_position);
            }
        }
        if self.portals.contains_key(&position) {
            neighbours.push(*self.portals.get(&position).unwrap());
        }
        neighbours
    }

    fn nearby_cells_with_level_adjustment(&self, position: Coord) -> Vec<(Coord, i64)> {
        let mut neighbours = Vec::new();
        for d in Direction::all() {
            let next_position = position.next(&d);
            if self.cells.contains(&next_position) {
                neighbours.push((next_position, 0));
            }
        }
        if self.portals.contains_key(&position) {
            neighbours.push((
                *self.portals.get(&position).unwrap(),
                match self.is_outer(position) {
                    true => -1,
                    false => 1,
                },
            ));
        }
        neighbours
    }

    fn is_outer(&self, position: Coord) -> bool {
        if position.x <= 3 {
            return true;
        }
        if position.x >= (self.width - 3) {
            return true;
        }
        if position.y <= 3 {
            return true;
        }
        if position.y >= (self.height - 3) {
            return true;
        }
        false
    }

    fn solve_maze(&self) -> u64 {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        let mut best_distance = (self.cells.len() + self.portals.len()) as u64;

        queue.push_back((self.entrance, 0));

        while !queue.is_empty() {
            let (position, distance_so_far) = queue.pop_back().unwrap();
            if distance_so_far > best_distance {
                continue; // sanity check, not really needed
            }

            if position == self.exit {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                }
                continue;
            }

            if distance_so_far < *distances.get(&position).unwrap_or(&best_distance) {
                distances.insert(position, distance_so_far);
            }

            for cell in self.nearby_cells(position) {
                if distances.contains_key(&cell)
                    && *distances.get(&cell).unwrap() <= (distance_so_far + 1)
                {
                    continue;
                }
                queue.push_back((cell, distance_so_far + 1));
            }
        }

        best_distance
    }

    fn solve_with_levels(&self) -> u64 {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        let mut best_distance = (self.cells.len() * self.portals.len()) as u64;

        queue.push_back((self.entrance, 0, 0));

        while !queue.is_empty() {
            let (position, level, distance_so_far) = queue.pop_front().unwrap();
            if distance_so_far > best_distance {
                continue; // sanity check, not really needed
            }

            if level < 0 || level > 30 {
                continue;
            }

            if position == self.exit && level == 0 {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                }
                continue;
            }

            if distance_so_far < *distances.get(&(position, level)).unwrap_or(&best_distance) {
                distances.insert((position, level), distance_so_far);
            }

            for (cell, level_adjustement) in self.nearby_cells_with_level_adjustment(position) {
                let new_level = level + level_adjustement;
                if distances.contains_key(&(cell, new_level))
                    && *distances.get(&(cell, new_level)).unwrap() <= (distance_so_far + 1)
                {
                    continue;
                }
                if level_adjustement > 0 {
                    queue.push_back((cell, new_level, distance_so_far + 1));
                } else {
                    queue.push_front((cell, new_level, distance_so_far + 1));
                }
            }
        }

        best_distance
    }
}

#[test]
fn test_solve_with_levels() {
    let s = "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "
        .to_string();
    let tunnel = TunnelMap::from_string(&s);
    assert!(tunnel.solve_with_levels() == 26);

    let s = "
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M          "
        .to_string();
    let tunnel = TunnelMap::from_string(&s);
    let path_len = tunnel.solve_with_levels();
    assert!(path_len == 396);
}

#[test]
fn test_solve_maze() {
    let s = "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "
        .to_string();
    let tunnel = TunnelMap::from_string(&s);
    let path_len = tunnel.solve_maze();
    assert!(path_len == 23);
}

#[test]
fn test_inner_outer() {
    let s = "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "
        .to_string();
    let tunnel = TunnelMap::from_string(&s);
    assert!(tunnel.is_outer(Coord { x: 9, y: 7 }) == false);
    assert!(tunnel.is_outer(Coord { x: 2, y: 16 }) == true);
    assert!(tunnel.is_outer(Coord { x: 11, y: 13 }) == false);
    assert!(tunnel.is_outer(Coord { x: 2, y: 9 }) == true);
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "North",
                Direction::South => "South",
                Direction::West => "West",
                Direction::East => "East",
            }
        )
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
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
