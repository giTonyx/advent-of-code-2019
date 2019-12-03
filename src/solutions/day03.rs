use crate::solutions::day03::Direction::{DOWN, LEFT, RIGHT};
use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

enum Direction {
    DOWN,
    UP,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn from(c: char) -> Direction {
        match c {
            'D' => DOWN,
            'U' => Direction::UP,
            'L' => LEFT,
            _ => RIGHT, // Assuming the input is well formed, or this should return a Result
        }
    }
}

pub struct Movement {
    direction: Direction,
    amount: u64,
}

impl Movement {
    pub fn from(s: String) -> Movement {
        Movement {
            direction: Direction::from(s.chars().nth(0).unwrap()),
            amount: s[1..].parse().unwrap(),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Vec<Movement>>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<Vec<Movement>> {
        let r = BufReader::new(r);
        r.lines()
            .map(|x| {
                x.unwrap()
                    .split(',')
                    .map(|x| x.to_string())
                    .map(|s| Movement::from(s))
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Vec<Vec<Movement>>) -> i64 {
        let first_points = navigate(&input[0]);
        let second_points = navigate(&input[1]);
        first_points
            .intersection(&second_points)
            .map(|p| p.x + p.y)
            .filter(|distance| !distance.eq(&0))
            .min()
            .unwrap()
    }

    fn solve_second(&self, input: &Vec<Vec<Movement>>) -> i64 {
        let first_points = navigate(&input[0]);
        let second_points = navigate(&input[1]);
        first_points
            .intersection(&second_points)
            .filter(|p| p.x + p.y > 0)
            .map(|p| navigate_until(&input[0], p) + navigate_until(&input[1], p))
            .min()
            .unwrap()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_one(&self, direction: &Direction) -> Point {
        match direction {
            Direction::UP => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::DOWN => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::LEFT => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::RIGHT => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn navigate(movements: &Vec<Movement>) -> HashSet<Point> {
    let mut points = HashSet::new();
    let mut current_point = Point { x: 0, y: 0 };
    for movement in movements {
        for _ in 1..=movement.amount {
            current_point = current_point.move_one(&movement.direction);
            points.insert(current_point.clone());
        }
    }
    points
}

fn navigate_until(movements: &Vec<Movement>, destination: &Point) -> i64 {
    let mut current_point = Point { x: 0, y: 0 };
    let mut distance = 0;
    for movement in movements {
        for _ in 1..=movement.amount {
            distance += 1;
            current_point = current_point.move_one(&movement.direction);
            if current_point == *destination {
                return distance;
            }
        }
    }
    0 // Here should actually be an error
}
