use crate::solver::Solver;
use num::integer::Integer;
use permutator::Combination;
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Moon>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .map(|l| l.unwrap())
            .map(|l| Moon::from_string(l))
            .collect::<Vec<Moon>>()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut moons = input.clone();

        for _ in 0..1000 {
            advance(&mut moons);
        }
        moons.iter().map(|m| m.energy()).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let period_x = find_period(input, |m: &Moon| m.x);
        let period_y = find_period(input, |m: &Moon| m.y);
        let period_z = find_period(input, |m: &Moon| m.z);

        period_x.lcm(&period_y).lcm(&period_z)
    }
}

fn find_period(input: &Vec<Moon>, f: fn(&Moon) -> i64) -> u64 {
    let mut moons = input.clone();
    let original_values = moons.iter().map(|m| f(m)).collect::<Vec<i64>>();
    let mut iterations = 1;
    loop {
        advance(&mut moons);
        iterations += 1;
        if moons.iter().map(|m| f(m)).collect::<Vec<i64>>() == original_values {
            break;
        }
    }
    iterations
}

fn advance(moons: &mut Vec<Moon>) {
    let indexes = vec![0, 1, 2, 3];

    let moons_clone = moons.clone();
    for index_couple in indexes.combination(2) {
        let i = **index_couple.get(0).unwrap() as usize;
        let j = **index_couple.get(1).unwrap() as usize;
        moons
            .get_mut(i)
            .unwrap()
            .update_velocity(moons_clone.get(j).unwrap());
        moons
            .get_mut(j)
            .unwrap()
            .update_velocity(moons_clone.get(i).unwrap());
    }

    for moon in moons.iter_mut() {
        moon.update_position();
    }
}

#[derive(Clone)]
pub struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    pub fn from_string(data: String) -> Moon {
        let parts = data[1..data.len() - 1].split(",").collect::<Vec<&str>>();
        let x = parts[0].split('=').collect::<Vec<&str>>()[1]
            .trim()
            .parse::<i64>()
            .unwrap();
        let y = parts[1].split('=').collect::<Vec<&str>>()[1]
            .trim()
            .parse::<i64>()
            .unwrap();
        let z = parts[2].split('=').collect::<Vec<&str>>()[1]
            .trim()
            .parse::<i64>()
            .unwrap();
        Moon {
            x: x,
            y: y,
            z: z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    pub fn energy(&self) -> u64 {
        ((self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())) as u64
    }

    pub fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    pub fn update_velocity(&mut self, other: &Moon) {
        if self.x > other.x {
            self.vx -= 1;
            //other.vx += 1;
        }
        if self.x < other.x {
            self.vx += 1;
            //other.vx -= 1;
        }
        if self.y > other.y {
            self.vy -= 1;
            //other.vy += 1;
        }
        if self.y < other.y {
            self.vy += 1;
            //other.vy -= 1;
        }
        if self.z > other.z {
            self.vz -= 1;
            //other.vz += 1;
        }
        if self.z < other.z {
            self.vz += 1;
            //other.vz -= 1;
        }
    }
}

#[test]
fn test_from_string() {
    let m = Moon::from_string("<x=19, y=-10, z=-7>".to_string());
    assert!(m.x == 19);
    assert!(m.y == -10);
    assert!(m.z == -7);
    assert!(m.vx == 0);
    assert!(m.vy == 0);
    assert!(m.vz == 0);
}
