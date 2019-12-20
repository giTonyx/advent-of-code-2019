use crate::coords::Coord;
use crate::solver::Solver;
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Coord>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        parse_map(
            &BufReader::new(r)
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|p| count_visible_points(p, input))
            .max()
            .unwrap() as u64
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let best_size = self.solve_first(input) as usize;
        let best_vec: Vec<&Coord> = input
            .iter()
            .filter(|p| count_visible_points(p, input) == best_size)
            .collect();
        let best_point = best_vec[0];
        vaporize(best_point, 200, input)
    }
}

fn vaporize(source: &Coord, count: usize, coords: &Vec<Coord>) -> u64 {
    let mut vaporized_points: Vec<(f64, &Coord)> = coords
        .iter()
        .filter(|p| has_los(source, p, coords))
        .map(|p| (source.slope(p), p))
        .collect();
    vaporized_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    if count > vaporized_points.len() {
        return vaporize(
            source,
            count - vaporized_points.len(),
            &coords
                .iter()
                .filter(|p| !has_los(source, p, coords))
                .map(|p| p.clone())
                .collect(),
        );
    }
    let point = vaporized_points[count - 1].1;
    point.x as u64 * 100 + point.y as u64
}

fn count_visible_points(source: &Coord, coords: &Vec<Coord>) -> usize {
    coords.iter().filter(|p| has_los(source, p, coords)).count()
}

fn has_los(source: &Coord, destination: &Coord, coords: &Vec<Coord>) -> bool {
    if source == destination {
        // Point does not have los to itself
        return false;
    }
    let slope = source.slope(&destination);
    for point in coords {
        if point == source {
            continue;
        }
        if point == destination {
            continue;
        }

        if source.slope(point) == slope && source.distance(point) < source.distance(destination) {
            return false;
        }
    }
    true
}

impl Coord {
    pub fn slope(&self, other: &Coord) -> f64 {
        let delta_x = -(self.x as f64 - other.x as f64);
        let delta_y = self.y as f64 - other.y as f64;
        match delta_x.atan2(delta_y).to_degrees() {
            x if x < 0f64 => 360f64 + x,
            x => x,
        }
    }

    pub fn distance(&self, other: &Coord) -> u8 {
        ((self.x as i8 - other.x as i8).abs() + (self.y as i8 - other.y as i8).abs()) as u8
    }
}

fn parse_map(string_data: &Vec<String>) -> Vec<Coord> {
    let mut coords = Vec::new();
    for y in 0..string_data.len() {
        let data = string_data.get(y).unwrap();
        for x in 0..data.len() {
            if data.chars().nth(x).unwrap() == '#' {
                coords.push(Coord {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    coords
}

#[test]
fn test_slope() {
    assert!(Coord { x: 10, y: 10 }.slope(&Coord { x: 10, y: 0 }) == 0f64);
    assert!(Coord { x: 10, y: 10 }.slope(&Coord { x: 20, y: 00 }) == 45f64);
    assert!(Coord { x: 10, y: 10 }.slope(&Coord { x: 20, y: 10 }) == 90f64);
    assert!(Coord { x: 10, y: 10 }.slope(&Coord { x: 10, y: 20 }) == 180f64);
    assert!(Coord { x: 10, y: 10 }.slope(&Coord { x: 0, y: 10 }) == 270f64);
}

#[test]
fn test_vaporize() {
    let mut input_data = Vec::new();
    input_data.push(".#....#####...#..".to_string());
    input_data.push("##...##.#####..##".to_string());
    input_data.push("##...#...#.#####.".to_string());
    input_data.push("..#.....X...###..".to_string());
    input_data.push("..#.#.....#....##".to_string());
    let coords = parse_map(&input_data);

    let source = Coord { x: 8, y: 3 };
    assert!(vaporize(&source, 1, &coords) == 801);
    assert!(vaporize(&source, 2, &coords) == 900);
    assert!(vaporize(&source, 3, &coords) == 901);
    assert!(vaporize(&source, 4, &coords) == 1000);
    assert!(vaporize(&source, 5, &coords) == 902);
}

#[test]
fn test_visible_points() {
    let mut input_data = Vec::new();
    input_data.push(".#..#".to_string());
    input_data.push(".....".to_string());
    input_data.push("#####".to_string());
    input_data.push("....#".to_string());
    input_data.push("...##".to_string());
    let coords = parse_map(&input_data);

    assert!(count_visible_points(&Coord { x: 3, y: 4 }, &coords) == 8);
    assert!(count_visible_points(&Coord { x: 4, y: 2 }, &coords) == 5);
    assert!(count_visible_points(&Coord { x: 4, y: 3 }, &coords) == 7);
    assert!(count_visible_points(&Coord { x: 0, y: 2 }, &coords) == 6);

    let mut input_data = Vec::new();
    input_data.push("......#.#.".to_string());
    input_data.push("#..#.#....".to_string());
    input_data.push("..#######.".to_string());
    input_data.push(".#.#.###..".to_string());
    input_data.push(".#..#.....".to_string());
    input_data.push("..#....#.#".to_string());
    input_data.push("#..#....#.".to_string());
    input_data.push(".##.#..###".to_string());
    input_data.push("##...#..#.".to_string());
    input_data.push(".#....####".to_string());
    let coords = parse_map(&input_data);

    assert!(count_visible_points(&Coord { x: 5, y: 8 }, &coords) == 33);
}

#[test]
fn test_has_los() {
    let mut input_data = Vec::new();
    input_data.push(".#..#".to_string());
    input_data.push(".....".to_string());
    input_data.push("#####".to_string());
    input_data.push("....#".to_string());
    input_data.push("...##".to_string());
    let coords = parse_map(&input_data);
    assert!(!has_los(
        &Coord { x: 3, y: 4 },
        &Coord { x: 1, y: 0 },
        &coords
    ));
    assert!(has_los(
        &Coord { x: 3, y: 4 },
        &Coord { x: 1, y: 2 },
        &coords
    ));
    assert!(has_los(
        &Coord { x: 3, y: 4 },
        &Coord { x: 4, y: 0 },
        &coords
    ));
    assert!(has_los(
        &Coord { x: 3, y: 4 },
        &Coord { x: 4, y: 4 },
        &coords
    ));
    assert!(has_los(
        &Coord { x: 4, y: 2 },
        &Coord { x: 4, y: 0 },
        &coords
    ));
}

#[test]
fn test_parse_map() {
    let mut input_data = Vec::new();
    input_data.push("..#".to_string());
    input_data.push("...".to_string());
    input_data.push(".#.".to_string());
    let coords = parse_map(&input_data);

    assert!(coords.len() == 2);
    assert!(coords.contains(&Coord { x: 2, y: 0 }));
    assert!(coords.contains(&Coord { x: 1, y: 2 }));
    assert!(!coords.contains(&Coord { x: 2, y: 1 }));
}
