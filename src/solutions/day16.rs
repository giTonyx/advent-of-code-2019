use crate::solver::Solver;
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut buffer = String::new();
        BufReader::new(r).read_to_string(&mut buffer).expect("");
        buffer.trim().to_string()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut pattern = string_to_digit_vector(input.clone());
        for _ in 0..100 {
            pattern = fft(&mut pattern);
        }
        vector_to_string(&pattern, 0, 8)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut pattern = string_to_digit_vector(input.repeat(10000));
        let offset = vector_to_string(&pattern, 0, 7).parse::<usize>().unwrap();
        for _ in 0..100 {
            for i in ((offset)..(pattern.len() - 2)).rev() {
                pattern[i] = (pattern[i] + pattern[i + 1]) % 10;
            }
        }
        vector_to_string(&pattern, offset, 8)
    }
}

fn vector_to_string(data: &Vec<i32>, start: usize, len: usize) -> String {
    data[start..start + len]
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}
#[test]
fn test_vector_to_string() {
    assert!(vector_to_string(&vec![1, 2, 3, 4, 5, 6], 0, 4) == "1234".to_string());
    assert!(vector_to_string(&vec![1, 2, 3, 4, 5, 6], 1, 4) == "2345".to_string());
}

fn string_to_digit_vector(data: String) -> Vec<i32> {
    data.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}

#[test]
fn test_string_parsing() {
    let vec = string_to_digit_vector("1234567890".to_string());
    assert!(vec.len() == 10);
    assert!(vec[1] == 2);
}

fn fft(pattern: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for i in 0..pattern.len() {
        output.push(transform_row(pattern, i as u64));
    }
    output
}

#[test]
fn test_fft() {
    assert!(
        vector_to_string(&fft(&string_to_digit_vector("12345678".to_string())), 0, 8)
            == "48226158".to_string()
    );
    assert!(
        vector_to_string(&fft(&string_to_digit_vector("03415518".to_string())), 0, 8)
            == "01029498".to_string()
    );
}

fn transform_row(pattern: &Vec<i32>, row: u64) -> i32 {
    let mut total = 0;
    for i in 0..pattern.len() {
        total += pattern[i] * pattern_digit(row, i as u64);
    }
    (total.abs()) % 10
}

fn pattern_digit(row: u64, column: u64) -> i32 {
    let period_len = 4 * (row + 1);
    let expanded_index = (column + 1) % period_len;
    let divided_index = expanded_index / (row + 1);
    match divided_index {
        0 => 0,
        1 => 1,
        2 => 0,
        _ => -1,
    }
}

#[test]
fn test_pattern_digit() {
    assert!(pattern_digit(0, 0) == 1);
    assert!(pattern_digit(0, 1) == 0);
    assert!(pattern_digit(1, 0) == 0);
    assert!(pattern_digit(1, 1) == 1);
}
