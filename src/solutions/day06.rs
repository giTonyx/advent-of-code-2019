use crate::solver::Solver;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

pub struct Orbit {
    source: String,
    orbital: String,
}

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(")") {
            None => Err(()),
            Some(split_index) => Ok(Orbit {
                source: s[0..split_index].to_string(),
                orbital: s[split_index + 1..].trim().to_string(),
            }),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Orbit>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| l.parse::<Orbit>().expect(""))
            .collect::<Vec<_>>()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut num_orbits = 0u64;
        let mut orbits: HashMap<&String, &String> = HashMap::new();
        for orbit in input {
            orbits.insert(&orbit.orbital, &orbit.source);
        }
        for orbit in input {
            num_orbits += 1 + count_orbits(&orbit.source, &orbits);
        }

        num_orbits
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let you_string = String::from("YOU");
        let san_string = String::from("SAN");

        let mut orbits: HashMap<&String, &String> = HashMap::new();
        for orbit in input {
            orbits.insert(&orbit.orbital, &orbit.source);
        }
        let mut you_object = orbits.get(&you_string).expect("");
        let mut san_object = orbits.get(&san_string).expect("");

        let mut you_distance = count_orbits(you_object, &orbits);
        let mut san_distance = count_orbits(san_object, &orbits);

        let mut hops = 0u64;

        while you_distance > san_distance {
            you_object = orbits.get(you_object).expect("");
            you_distance -= 1;
            hops += 1;
        }
        while san_distance > you_distance {
            san_object = orbits.get(san_object).expect("");
            san_distance -= 1;
            hops += 1;
        }
        while you_object != san_object {
            san_object = orbits.get(san_object).expect("");
            you_object = orbits.get(you_object).expect("");
            hops += 2;
        }
        hops
    }
}

fn count_orbits(name: &String, orbits: &HashMap<&String, &String>) -> u64 {
    match orbits.get(name) {
        None => 0,
        Some(parent) => 1 + count_orbits(parent, orbits),
    }
}

#[test]
fn test_orbit_from_string() {
    let o1 = "A)B".parse::<Orbit>().expect("");
    assert!(o1.source == "A".to_string());
    assert!(o1.orbital == "B".to_string());
    let o2 = "ABC)DEF".parse::<Orbit>().expect("");
    assert!(o2.source == "ABC".to_string());
    assert!(o2.orbital == "DEF".to_string());
}
