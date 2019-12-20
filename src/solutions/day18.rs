use crate::coords::{Coord, Direction};
use crate::solver::Solver;
use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut buffer = String::new();
        BufReader::new(r).read_to_string(&mut buffer).expect("");
        buffer
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let maze = Maze::from_string(input.clone());
        maze.take_all_keys()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let maze = MultiMaze::from_string(input.clone());
        maze.take_all_keys()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct MultiState {
    keys: KeySet,
    positions: Vec<Coord>,
}

impl MultiState {
    pub fn from_coord(coord: Coord) -> MultiState {
        let mut positions = Vec::new();
        positions.push(Coord {
            x: coord.x - 1,
            y: coord.y - 1,
        });
        positions.push(Coord {
            x: coord.x - 1,
            y: coord.y + 1,
        });
        positions.push(Coord {
            x: coord.x + 1,
            y: coord.y - 1,
        });
        positions.push(Coord {
            x: coord.x + 1,
            y: coord.y + 1,
        });
        MultiState {
            keys: KeySet::new(),
            positions: positions,
        }
    }

    fn find_distance_and_index(
        &self,
        target: Coord,
        distances: &HashMap<(Coord, Coord), i64>,
    ) -> (i64, usize) {
        for idx in 0..self.positions.len() {
            let position = self.positions[idx];
            if distances.contains_key(&(position, target)) {
                return (*distances.get(&(position, target)).unwrap(), idx);
            }
        }
        (-1, 0) // ERROR
    }

    fn moving_to(&self, robot_idx: usize, destination: Coord, new_key: char) -> MultiState {
        let mut new_positions = self.positions.clone();
        new_positions[robot_idx] = destination;
        MultiState {
            keys: self.keys.with(new_key),
            positions: new_positions,
        }
    }
}

struct MultiMaze {
    cells: HashMap<Coord, Tile>,
    doors: HashMap<Coord, char>,
    keys: HashMap<Coord, char>,
    state: MultiState,
}

impl MultiMaze {
    fn take_all_keys(&self) -> i64 {
        let mut best_distance = (self.cells.len() * self.keys.len()) as i64;
        let mut all_best_distances = HashMap::new();

        let distances = self.pre_calculate_distances();
        let unlocks = self.calculate_unlocks();

        let mut queue = VecDeque::new();
        queue.push_back((self.state.clone(), 0));

        while !queue.is_empty() {
            let (state, distance_so_far) = queue.pop_back().unwrap();

            // If we already get there at same distance, we can skip
            if distance_so_far >= *all_best_distances.get(&state).unwrap_or(&best_distance) {
                continue;
            }
            all_best_distances.insert(state.clone(), distance_so_far);

            if state.keys.keys.len() == self.keys.len() {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                    continue;
                }
            }

            // See where we can go from here
            for key in self.possible_keys(&state.keys, &unlocks) {
                let key_position = self.find_key_coord(key).unwrap();
                let (distance, idx) = state.find_distance_and_index(key_position, &distances);

                queue.push_back((
                    state.moving_to(idx, key_position, key),
                    distance_so_far + distance,
                ));
            }
        }

        best_distance
    }

    fn possible_keys(&self, seen_keys: &KeySet, unlocks: &HashMap<char, Vec<char>>) -> Vec<char> {
        let mut keys = Vec::new();

        for k in self.all_keys() {
            if seen_keys.has(k) {
                continue;
            }
            if unlocks
                .get(&k)
                .unwrap()
                .iter()
                .filter(|d| !seen_keys.has(**d))
                .collect::<Vec<&char>>()
                .len()
                == 0
            {
                keys.push(k);
            }
        }

        keys
    }
    fn calculate_unlocks(&self) -> HashMap<char, Vec<char>> {
        let mut unlocks = HashMap::new();
        let mut queue = VecDeque::new();

        for position in &self.state.positions {
            let mut seen = HashSet::new();
            seen.insert(*position);
            queue.push_back((*position, Vec::new()));

            while !queue.is_empty() {
                let (coord, doors_opened) = queue.pop_front().unwrap();
                if self.keys.contains_key(&coord) {
                    unlocks.insert(*self.keys.get(&coord).unwrap(), doors_opened.clone());
                }
                let mut new_doors_opened = doors_opened.clone();
                if self.doors.contains_key(&coord) {
                    new_doors_opened.push(*self.doors.get(&coord).unwrap());
                }

                for d in Direction::all() {
                    let next_coord = coord.next(&d);
                    if seen.contains(&next_coord) {
                        continue;
                    }
                    if !self.cells.contains_key(&next_coord) {
                        continue;
                    }
                    if *self.cells.get(&next_coord).unwrap() == Tile::Wall {
                        continue;
                    }
                    seen.insert(next_coord);
                    queue.push_back((next_coord, new_doors_opened.clone()));
                }
            }
        }

        unlocks
    }
    fn pre_calculate_distances(&self) -> HashMap<(Coord, Coord), i64> {
        let mut distances = HashMap::new();
        for key in self.all_keys() {
            let destination = self.find_key_coord(key).unwrap();

            self.state.positions.iter().for_each(|start| {
                let distance = self.greedy_distance(*start, destination);
                if distance > 0 {
                    distances.insert((*start, destination), distance);
                }
            });
            self.keys.iter().for_each(|(start, _v)| {
                let distance = self.greedy_distance(*start, destination);
                if distance > 0 {
                    distances.insert((*start, destination), distance);
                }
            });
        }
        distances
    }

    pub fn from_string(s: String) -> MultiMaze {
        let mut cells = HashMap::new();
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();
        let mut entrance = Coord { x: 0, y: 0 };

        let mut y = -1;
        for line in s.split('\n').map(|l| l.trim()) {
            y += 1;
            let mut x = -1;
            for c in line.chars() {
                x += 1;
                let position = Coord { x: x, y: y };
                if cells.contains_key(&position) {
                    continue;
                }
                match c {
                    '#' => {
                        cells.insert(position, Tile::Wall);
                    }
                    '.' => {
                        cells.insert(position, Tile::Empty);
                    }
                    '@' => {
                        cells.insert(position, Tile::Wall);
                        cells.insert(Coord { x: x + 1, y: y }, Tile::Wall);
                        cells.insert(Coord { x: x - 1, y: y }, Tile::Wall);
                        cells.insert(Coord { x: x, y: y + 1 }, Tile::Wall);
                        cells.insert(Coord { x: x, y: y - 1 }, Tile::Wall);
                        entrance = position;
                    }
                    'a'..='z' => {
                        cells.insert(position, Tile::Empty);
                        keys.insert(position, c);
                    }
                    'A'..='Z' => {
                        cells.insert(position, Tile::Empty);
                        doors.insert(position, c.to_ascii_lowercase());
                    }
                    _ => {
                        println!("ERROR: Unexptected value on the map: {}", c);
                    }
                }
            }
        }
        MultiMaze {
            cells: cells,
            doors: doors,
            keys: keys,
            state: MultiState::from_coord(entrance),
        }
    }

    fn all_keys(&self) -> Vec<char> {
        self.keys.iter().map(|(_, v)| *v).collect::<Vec<char>>()
    }
    fn find_key_coord(&self, key: char) -> Option<Coord> {
        for (coord, k) in self.keys.iter() {
            if *k == key {
                return Some(*coord);
            }
        }
        None
    }

    // Does not take doors into account
    fn greedy_distance(&self, start: Coord, end: Coord) -> i64 {
        let mut best_distance = self.cells.len() as i64;
        let mut found = false;

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut distances = HashMap::new();
        distances.insert(start, 0);

        while !queue.is_empty() {
            let (coord, distance_so_far) = queue.pop_front().unwrap();

            // Are we arrived?
            if coord == end {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                }
                found = true;
                continue;
            }

            // Are we too far?
            if (distance_so_far + 1) >= best_distance {
                continue;
            }

            // Move
            for d in Direction::all() {
                let next_coord = coord.next(&d);
                if !self.cells.contains_key(&next_coord) {
                    continue;
                }
                if *self.cells.get(&next_coord).unwrap() == Tile::Wall {
                    continue;
                }
                if distances.contains_key(&next_coord)
                    && *distances.get(&next_coord).unwrap() <= (distance_so_far + 1)
                {
                    continue;
                }
                distances.insert(next_coord, distance_so_far + 1);
                queue.push_back((next_coord, distance_so_far + 1));
            }
        }
        if !found {
            return -1;
        }
        best_distance
    }
}

impl fmt::Debug for MultiMaze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        let max_x = self.cells.iter().map(|(k, _)| k.x).max().unwrap();
        let max_y = self.cells.iter().map(|(k, _)| k.y).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x: x, y: y };
                if self.state.positions.contains(&coord) {
                    buffer.push('@');
                    continue;
                }
                if *self.cells.get(&coord).unwrap() == Tile::Wall {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

#[test]
fn test_multi_door_unlocks() {
    let s = "#######\n\
             #a.#Cd#\n\
             ##...##\n\
             ##.@.##\n\
             ##...##\n\
             #cB#Ab#\n\
             #######"
        .to_string();
    let maze = MultiMaze::from_string(s);
    let unlocks = maze.calculate_unlocks();
    println!("{:?}", unlocks);
}

#[test]
fn test_multi_take_all_keys() {
    let s = "#######\n\
             #a.#Cd#\n\
             ##...##\n\
             ##.@.##\n\
             ##...##\n\
             #cB#Ab#\n\
             #######"
        .to_string();
    let maze = MultiMaze::from_string(s);
    assert!(maze.take_all_keys() == 8);

    let s = "#############\n\
             #g#f.D#..h#l#\n\
             #F###e#E###.#\n\
             #dCba...BcIJ#\n\
             #####.@.#####\n\
             #nK.L...G...#\n\
             #M###N#H###.#\n\
             #o#m..#i#jk.#\n\
             #############"
        .to_string();
    let maze = MultiMaze::from_string(s);
    assert!(maze.take_all_keys() == 72);
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct KeySet {
    keys: String,
}

impl KeySet {
    pub fn new() -> KeySet {
        KeySet {
            keys: String::new(),
        }
    }

    pub fn from(s: String) -> KeySet {
        KeySet { keys: s }
    }

    fn has(&self, key: char) -> bool {
        self.keys.contains(key)
    }

    fn with(&self, key: char) -> KeySet {
        let mut char_vector = self.keys.chars().collect::<Vec<char>>();
        char_vector.push(key);
        char_vector.sort();
        let new_string = char_vector.iter().collect();
        KeySet { keys: new_string }
    }
}

struct Maze {
    cells: HashMap<Coord, Tile>,
    doors: HashMap<Coord, char>,
    keys: HashMap<Coord, char>,
    entrance: Coord,
}

impl Maze {
    pub fn from_string(s: String) -> Maze {
        let mut cells = HashMap::new();
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();
        let mut entrance = Coord { x: 0, y: 0 };

        let mut y = -1;
        for line in s.split('\n').map(|l| l.trim()) {
            y += 1;
            let mut x = -1;
            for c in line.chars() {
                x += 1;
                let position = Coord { x: x, y: y };
                match c {
                    '#' => {
                        // We don't really need the walls
                        //cells.insert(position, Tile::Wall);
                    }
                    '.' => {
                        cells.insert(position, Tile::Empty);
                    }
                    '@' => {
                        cells.insert(position, Tile::Empty);
                        entrance = position;
                    }
                    'a'..='z' => {
                        cells.insert(position, Tile::Empty);
                        keys.insert(position, c);
                    }
                    'A'..='Z' => {
                        cells.insert(position, Tile::Empty);
                        doors.insert(position, c.to_ascii_lowercase());
                    }
                    _ => {
                        println!("ERROR: Unexptected value on the map: {}", c);
                    }
                }
            }
        }
        Maze {
            cells: cells,
            doors: doors,
            keys: keys,
            entrance: entrance,
        }
    }
    fn all_keys(&self) -> Vec<char> {
        self.keys.iter().map(|(_, v)| *v).collect::<Vec<char>>()
    }

    fn find_key_coord(&self, key: char) -> Option<Coord> {
        for (coord, k) in self.keys.iter() {
            if *k == key {
                return Some(*coord);
            }
        }
        None
    }

    fn take_all_keys(&self) -> i64 {
        let mut queue = VecDeque::new();
        queue.push_back((self.entrance, "".to_string(), 0));
        let target_size = self.all_keys().len();
        let mut best_distance = (self.cells.len() * target_size) as i64;
        let mut all_best_distances = HashMap::new();

        let doors_needed = self.doors_for_keys();
        let distances = self.pre_calculate_distances();

        while !queue.is_empty() {
            let (position, seen_keys_string, distance_so_far) = queue.pop_back().unwrap();

            let distance_key = (seen_keys_string.clone(), position);
            if distance_so_far
                >= *all_best_distances
                    .get(&distance_key)
                    .unwrap_or(&best_distance)
            {
                continue;
            }
            all_best_distances.insert(distance_key.clone(), distance_so_far);

            if seen_keys_string.len() == target_size {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                }
                continue;
            }
            let key_set = KeySet::from(seen_keys_string);

            let mut next_keys = self
                .possible_keys(&key_set, &doors_needed)
                .iter()
                .map(|key| {
                    let new_position = self.find_key_coord(*key).unwrap();
                    let distance = *distances.get(&(position, new_position)).unwrap();
                    (distance, new_position, *key)
                })
                .collect::<Vec<(i64, Coord, char)>>();
            next_keys.sort_by(|(a, _, _), (b, _, _)| b.partial_cmp(a).unwrap());

            for (distance, new_position, key) in next_keys {
                if distance_so_far + distance > best_distance {
                    continue;
                }
                let new_seen_keys = key_set.with(key).keys;

                queue.push_back((new_position, new_seen_keys, distance_so_far + distance));
            }
        }
        best_distance
    }

    fn doors_for_keys(&self) -> HashMap<char, Vec<char>> {
        let mut doors = HashMap::new();

        let mut seen = HashSet::new();
        seen.insert(self.entrance);

        let mut queue = VecDeque::new();
        queue.push_back((self.entrance, Vec::new()));

        while !queue.is_empty() {
            let (coord, doors_opened) = queue.pop_front().unwrap();

            if self.keys.contains_key(&coord) {
                doors.insert(*self.keys.get(&coord).unwrap(), doors_opened.clone());
            }

            let mut new_doors_opened = doors_opened.clone();
            if self.doors.contains_key(&coord) {
                new_doors_opened.push(*self.doors.get(&coord).unwrap());
            }

            for d in Direction::all() {
                let next_coord = coord.next(&d);
                if seen.contains(&next_coord) {
                    continue;
                }
                if !self.cells.contains_key(&next_coord) {
                    continue;
                }
                seen.insert(next_coord);
                queue.push_back((next_coord, new_doors_opened.clone()));
            }
        }

        doors
    }

    fn possible_keys(
        &self,
        seen_keys: &KeySet,
        doors_needed: &HashMap<char, Vec<char>>,
    ) -> Vec<char> {
        let mut keys = Vec::new();

        for k in self.all_keys() {
            if seen_keys.has(k) {
                continue;
            }
            if doors_needed
                .get(&k)
                .unwrap()
                .iter()
                .filter(|d| !seen_keys.has(**d))
                .collect::<Vec<&char>>()
                .len()
                == 0
            {
                keys.push(k);
            }
        }

        keys
    }

    // Does not take doors into account
    fn greedy_distance(&self, start: Coord, end: Coord) -> i64 {
        let mut best_distance = self.cells.len() as i64;

        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut distances = HashMap::new();
        distances.insert(start, 0);

        while !queue.is_empty() {
            let (coord, distance_so_far) = queue.pop_front().unwrap();

            // Are we arrived?
            if coord == end {
                if distance_so_far < best_distance {
                    best_distance = distance_so_far;
                }
                continue;
            }

            // Are we too far?
            if (distance_so_far + 1) >= best_distance {
                continue;
            }

            // Move
            for d in Direction::all() {
                let next_coord = coord.next(&d);
                if !self.cells.contains_key(&next_coord) {
                    continue;
                }
                if distances.contains_key(&next_coord)
                    && *distances.get(&next_coord).unwrap() <= (distance_so_far + 1)
                {
                    continue;
                }
                distances.insert(next_coord, distance_so_far + 1);
                queue.push_back((next_coord, distance_so_far + 1));
            }
        }
        best_distance
    }

    fn pre_calculate_distances(&self) -> HashMap<(Coord, Coord), i64> {
        let mut distances = HashMap::new();
        for key in self.all_keys() {
            let destination = self.find_key_coord(key).unwrap();
            distances.insert(
                (self.entrance, destination),
                self.greedy_distance(self.entrance, destination),
            );
            self.keys.iter().for_each(|(start, _v)| {
                distances.insert(
                    (*start, destination),
                    self.greedy_distance(*start, destination),
                );
            });
        }
        distances
    }
}

#[test]
fn test_pre_calculate() {
    let m = "#################\n\
             #i.G..c...e..H.p#\n\
             ########.########\n\
             #j.A..b...f..D.o#\n\
             ########@########\n\
             #k.E..a...g..B.n#\n\
             ########.########\n\
             #l.F..d...h..C.m#\n\
             #################"
        .to_string();
    let maze = Maze::from_string(m);
    let distances = maze.pre_calculate_distances();
    assert!(distances.len() == 272);
}

#[test]
fn test_possible_keys() {
    let m = "########################\n\
             #@..............ac.GI.b#\n\
             ###d#e#f################\n\
             ###A#B#C################\n\
             ###g#h#i################\n\
             ########################"
        .to_string();
    let maze = Maze::from_string(m);
    let keys = maze.possible_keys(&KeySet::new(), &maze.doors_for_keys());
    assert!(keys.len() == 5);

    let m = "#########\n\
             #bcA.@.a#\n\
             #########"
        .to_string();
    let maze = Maze::from_string(m);
    let mut seen_keys = KeySet::new();
    assert!(maze.possible_keys(&seen_keys, &maze.doors_for_keys()).len() == 1);
    seen_keys = seen_keys.with('a');
    assert!(maze.possible_keys(&seen_keys, &maze.doors_for_keys()).len() == 2);
}

#[test]
fn test_doors_for_keys() {
    let m = "########################\n\
             #@..............ac.GI.b#\n\
             ###d#e#f################\n\
             ###A#B#C################\n\
             ###g#h#i################\n\
             ########################"
        .to_string();
    let maze = Maze::from_string(m);
    maze.doors_for_keys();
}

#[test]
fn test_take_all_keys() {
    let m = "########################\n\
             #@..............ac.GI.b#\n\
             ###d#e#f################\n\
             ###A#B#C################\n\
             ###g#h#i################\n\
             ########################"
        .to_string();
    let maze = Maze::from_string(m);
    assert!(maze.take_all_keys() == 81);

    let m = "########################\n\
             #...............b.C.D.f#\n\
             #.######################\n\
             #.....@.a.B.c.d.A.e.F.g#\n\
             ########################"
        .to_string();
    let maze = Maze::from_string(m);
    assert!(maze.take_all_keys() == 132);

    let m = "#################\n\
             #i.G..c...e..H.p#\n\
             ########.########\n\
             #j.A..b...f..D.o#\n\
             ########@########\n\
             #k.E..a...g..B.n#\n\
             ########.########\n\
             #l.F..d...h..C.m#\n\
             #################"
        .to_string();
    let maze = Maze::from_string(m);
    assert!(maze.take_all_keys() == 136);
}

#[test]
fn test_maze_all_keys() {
    let m = "#########\n\
             #b.A.@.a#\n\
             #########"
        .to_string();
    let maze = Maze::from_string(m);
    assert!(maze.all_keys().len() == 2);
}

#[test]
fn test_maze_from_string() {
    let m = "#########\n\
             #b.A.@.a#\n\
             #########"
        .to_string();
    let maze = Maze::from_string(m);
    assert!(maze.keys.len() == 2);
    assert!(maze.doors.len() == 1);
    // assert!(maze.cells.len() == 27); // not passing anymore becaue of wall optimization
    assert!(maze.entrance.x == 5);
    assert!(maze.entrance.y == 1);
}
