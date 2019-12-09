use crate::solver::Solver;
use std::io::{BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = u64;
    type Output2 = String;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut buffer = String::new();
        BufReader::new(r).read_to_string(&mut buffer).unwrap();
        buffer.clone()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let frame_size = 25 * 6;
        let mut frame_counter = 0;

        let mut max_zeroes = frame_size as u64;
        let mut max_multiply = 0;

        while frame_counter * frame_size < (input.len() - 1) {
            let substring = subframe(&input, frame_size, frame_counter);
            let zeroes = count(substring, '0');
            if zeroes < max_zeroes {
                max_zeroes = zeroes;
                max_multiply = count(substring, '1') * count(substring, '2');
            }
            frame_counter += 1;
        }

        max_multiply
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let frame_size = 25 * 6;
        let mut frame_counter = 0;

        let mut top_pixels = subframe(&input, frame_size, 0).to_string();

        while frame_counter * frame_size < (input.len() - 1) {
            let substring = subframe(&input, frame_size, frame_counter).to_string();
            top_pixels = merge_frames(&top_pixels, &substring);
            frame_counter += 1;
        }

        display(&top_pixels, 25, 6);
        "FPUAR".to_string() // by looking at the displayed image
    }
}

fn display(data: &String, width: usize, height: usize) {
    let chars: Vec<char> = data.chars().collect();

    for h in 0..height {
        for w in 0..width {
            if chars[h * width + w] == '1' {
                print!("*")
            } else {
                print!(" ")
            }
        }
        println!("");
    }
}

fn subframe(data: &String, frame_size: usize, frame_counter: usize) -> &str {
    &data[frame_size * frame_counter..=((frame_counter + 1) * frame_size - 1)]
}

fn count(data: &str, digit: char) -> u64 {
    let mut total = 0;
    for c in data.chars() {
        if c == digit {
            total += 1
        }
    }
    total
}

fn merge_frames(top: &String, bottom: &String) -> String {
    let mut top_data: Vec<char> = top.chars().collect();
    let bottom_data: Vec<char> = bottom.chars().collect();

    for i in 1..top.len() {
        if top_data[i] == '2' {
            top_data[i] = bottom_data[i];
        }
    }
    let s: String = top_data.into_iter().collect();
    s
}

#[test]
fn test_subframe() {
    assert!(subframe(&"123456789012".to_string(), 6, 0) == "123456");
    assert!(subframe(&"123456789012".to_string(), 6, 1) == "789012");
}

#[test]
fn test_count() {
    assert!(count("123456789012", '0') == 1);
    assert!(count("123456789012", '1') == 2);
}
