use crate::solver::Solver;
use std::io::{Read, BufReader};

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
        let operations = Shuffle::from_multiple_strings(input);
        let mut position = 2019;
        for op in operations{
            position = op.move_position(position, 10007 );
        }
        position
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let operations = Shuffle::from_multiple_strings(input);
        let mut position = 2020;
        let deck_size = 119315717514047;
        let iterations = 101741582076661u64;
        //let period = find_period(&operations, position, deck_size);
        //println!("Period: {}", period);
        for _ in 0..iterations {
            for op in operations.clone(){
                position = op.move_position(position, deck_size );
            }
        }
        position
    }
}

fn find_period(operations: &Vec<Shuffle>, position: u64, deck_size: u64) -> u64 {
    let mut current_position = position;
    let mut period = 0;
    loop{
        for op in operations.clone(){
            current_position = op.move_position(current_position, deck_size );
        }
        period += 1;
        if current_position == position {break}
    }
    period
}

#[derive(PartialEq, Clone)]
enum Operation {
    NewStack,
    Cut,
    Increment,
}

#[derive(Clone)]
struct Shuffle {
    operation: Operation,
    value: i64,
}

impl Shuffle {
    pub fn from_string(data: String) -> Shuffle{
        if data.contains("deal into new stack") {
            return Shuffle{operation: Operation::NewStack, value:0}
        }
        let value = data.split(' ').next_back().unwrap().parse::<i64>().unwrap();
        if data.contains("cut") {
            return Shuffle{operation: Operation::Cut, value: value};
        }
        Shuffle{operation:Operation::Increment, value:value}
    }

    pub fn from_multiple_strings(data: &String) -> Vec<Shuffle>{
        data.trim().to_string().split('\n').map(|l| Shuffle::from_string(l.trim().to_string())).collect::<Vec<Shuffle>>()
    }

    fn move_position(&self, position: u64, deck_size:u64) -> u64 {
        match self.operation{
            Operation::NewStack => {deck_size - (position + 1) },
            Operation::Cut => {if self.value > 0 {
                let value = self.value as u64;
                if position < value as u64 {deck_size - value + position} else {position - value }
            } else {
                let value = self.value.abs() as u64;
                if position < (deck_size - value) {position + value} else {position - (deck_size - value)}
            }},
            Operation::Increment => {(position * self.value as u64) % deck_size},
        }
    }
}

#[test]
fn test_multiple_strings(){
    let s = "deal with increment 7
deal into new stack
deal into new stack
".to_string();
    let v = Shuffle::from_multiple_strings(&s);
    assert!(v.len() == 3);
}

#[test]
fn test_move_position() {
    let s = Shuffle::from_string("deal into new stack".to_string());
    assert!(s.move_position(0, 10) == 9);
    assert!(s.move_position(4, 10) == 5);
    let s = Shuffle::from_string("deal with increment 3".to_string());
    assert!(s.move_position(0, 10) == 0);
    assert!(s.move_position(1, 10) == 3);
    assert!(s.move_position(7, 10) == 1);
    assert!(s.move_position(9, 10) == 7);
    let s = Shuffle::from_string("cut 3".to_string());
    assert!(s.move_position(0, 10) == 7);
    assert!(s.move_position(3, 10) == 0);
    let s = Shuffle::from_string("cut -4".to_string());
    assert!(s.move_position(0, 10) == 4);
    assert!(s.move_position(6, 10) == 0);

}

#[test]
fn test_shuffle_from_string(){
    let s = Shuffle::from_string("deal into new stack".to_string());
    assert!(s.operation == Operation::NewStack);
    let s = Shuffle::from_string("deal with increment 29".to_string());
    assert!(s.operation == Operation::Increment);
    assert!(s.value == 29);
    let s = Shuffle::from_string("cut -4398".to_string());
    assert!(s.operation == Operation::Cut);
    assert!(s.value == -4398);
}