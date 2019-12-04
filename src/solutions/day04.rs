use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (i64, i64);
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> (i64, i64) {
        let r = BufReader::new(r);
        let all_input_numbers: Vec<i64> = r
            .split(b'-')
            .flatten()
            .filter_map(|v| String::from_utf8(v).ok())
            .filter_map(|v| v.to_owned().trim().to_string().parse().ok())
            .collect();
        (all_input_numbers[0], all_input_numbers[1])
    }

    fn solve_first(&self, input: &(i64, i64)) -> i64 {
        let (lower_bound, upper_bound) = input;
        (111111..=999999)
            .filter(|n| n >= lower_bound && n <= upper_bound)
            .filter(|n| non_decreasing(n))
            .filter(|n| has_double(n))
            .count() as i64
    }

    fn solve_second(&self, input: &(i64, i64)) -> i64 {
        let (lower_bound, upper_bound) = input;
        (111111..=999999)
            .filter(|n| n >= lower_bound && n <= upper_bound)
            .filter(|n| non_decreasing(n))
            .filter(|n| has_double(n))
            .filter(|n| has_simple_double(n))
            .count() as i64
    }
}

fn non_decreasing(number: &i64) -> bool {
    let mut current_digit = number % 10;
    for power in 2..=6 {
        let digit = (number % (10i64.pow(power))) / 10i64.pow(power - 1);
        if digit > current_digit {
            return false;
        }
        current_digit = digit;
    }
    true
}
fn has_double(number: &i64) -> bool {
    let mut current_digit = number % 10;
    for power in 2..=6 {
        let digit = (number % (10i64.pow(power))) / 10i64.pow(power - 1);
        if digit == current_digit {
            return true;
        }
        current_digit = digit;
    }
    false
}

fn has_simple_double(number: &i64) -> bool {
    let temp = number.to_string();
    let number_str = temp.as_bytes();
    let mut current_digit = number_str[0];
    let mut current_size = 1;

    for idx in 1..=5 {
        if number_str[idx] == current_digit {
            current_size += 1
        } else {
            if current_size == 2 {
                return true;
            }
            current_size = 1;
            current_digit = number_str[idx];
        }
    }
    current_size == 2
}

#[test]
fn test_has_simple_double() {
    assert!(has_simple_double(&111122) == true);
    assert!(has_simple_double(&111222) == false);
}
