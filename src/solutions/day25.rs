use crate::solver::Solver;
use std::io::Read;
use crate::intcode::{IntCode, IntInput, read_input};
use std::{io, thread};
use std::time::Duration;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        //interactive_run(input);
        get_password(input);
        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        0
    }
}

fn get_password(program: &Vec<i64>) {
    let commands = "south
    south
    take hypercube
    north
    north
    north
    take tambourine
    east
    take astrolabe
    south
    take shell
    north
    east
    north
    take klein bottle
    north
    take easter egg
    south
    south
    west
    west
    south
    west
    take dark matter
    west
    north
    west
    take coin
    south
    inv
    ".to_string();
    // Found with brute force
    let item1 = 0;
    let item2 = 0;
    let item3 = 1;
    let item4 = 1;
    let item5 = 1;
    let item6 = 1;
    let item7 = 0;
    let item8 = 0;

                                    println!("Trying: {},{},{},{},{},{},{},{}", item1,item2,item3,item4,item5,item6,item7,item8);
                                    let mut intcode = IntCode::new(program);

                                    for c in commands.split('\n'){
                                        send_command(&mut intcode, c.trim().to_string());
                                    }

                                    if item1 == 1 {send_command(&mut intcode, "drop hypercube".to_string());}
                                    if item2 == 1 {send_command(&mut intcode, "drop tambourine".to_string());}
                                    if item3 == 1 {send_command(&mut intcode, "drop astrolabe".to_string());}
                                    if item4 == 1 {send_command(&mut intcode, "drop shell".to_string());}
                                    if item5 == 1 {send_command(&mut intcode, "drop klein bottle".to_string());}
                                    if item6 == 1 {send_command(&mut intcode, "drop easter egg".to_string());}
                                    if item7 == 1 {send_command(&mut intcode, "drop dark matter".to_string());}
                                    if item8 == 1 {send_command(&mut intcode, "drop coin".to_string());}
                                    send_command(&mut intcode, "south".to_string());

}

fn send_command(intcode: &mut IntCode, data: String ) {
    for c in data.chars()  {
        intcode.input.push(c as i64);
    }
    intcode.input.push(10);
    let mut output = IntInput::new();
    intcode.advance(&mut output);

    // Print output
    let mut buffer = String::new();
    while output.has_input(){
        let c = (output.get() as u8)  as char;
        if c == '\n' {
            //if buffer.contains("Alert!"){println!("{}",buffer)};
            println!("{}", buffer);
            buffer = "".to_string();}
        buffer.push(c);
    }

}

fn interactive_run(program: &Vec<i64>) {
    let mut intcode = IntCode::new(program);
    let mut output = IntInput::new();
    let mut quit = false;

    while !quit {
        // Advance
        intcode.advance(&mut output);

        // Print output
        while output.has_input(){
            let c = (output.get() as u8)  as char;
            print!("{}", c);
        }
        println!("");

        // Read input
        loop {
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    buffer = buffer.trim().to_string();
                    for c in buffer.chars() {
                        intcode.input.push(c as i64);
                    }
                    intcode.input.push(10);
                    if buffer == "q".to_string() { quit = true };
                    break
                },
                Err(_) => { std::thread::sleep(Duration::from_millis(200)); },
            }
        }
    }
}