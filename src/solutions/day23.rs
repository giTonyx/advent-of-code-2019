use crate::intcode::{read_input, IntCode, IntInput};
use crate::solver::Solver;
use std::collections::HashMap;
use std::io;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<i64>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: io::Read>(&self, r: R) -> Vec<i64> {
        read_input(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let num_computers = 50;
        let mut computers = Vec::new();
        for i in 0..num_computers {
            computers.push(IntCode::new(input));
            computers[i].input.set_nonblocking();
            computers[i].input.push(i as i64);
        }

        let mut last_y = 0;

        let mut packets = PacketOutput::new();
        loop {
            let mut should_stop = false;
            let mut output = IntInput::new();
            for i in 0..num_computers {
                computers[i].step(&mut output);
                while output.has_input() {
                    let val = output.get();
                    let (destination, x, y) = packets.push_packet_piece(i as i64, val);
                    if destination == 255 {
                        should_stop = true;
                        last_y = y;
                        break;
                    }
                    if destination < 0 {
                        continue;
                    }
                    computers[destination as usize].input.push(x);
                    computers[destination as usize].input.push(y);
                }
            }

            if should_stop {
                break;
            }
        }

        last_y
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let num_computers = 50;
        let mut computers = Vec::new();
        for i in 0..num_computers {
            computers.push(IntCode::new(input));
            computers[i].input.set_nonblocking();
            computers[i].input.push(i as i64);
        }

        let mut nat_x = 0;
        let mut nat_y = 0;
        let mut steps_since_output = 0;

        let mut prev_y_sent = -1;

        let mut packets = PacketOutput::new();
        loop {
            let mut should_stop = false;
            let mut output = IntInput::new();
            steps_since_output += 1;
            for i in 0..num_computers {
                computers[i].step(&mut output);
                while output.has_input() {
                    steps_since_output = 0;
                    let val = output.get();
                    let (destination, x, y) = packets.push_packet_piece(i as i64, val);
                    if destination == 255 {
                        nat_x = x;
                        nat_y = y;
                        break;
                    }
                    if destination < 0 {
                        continue;
                    }
                    computers[destination as usize].input.push(x);
                    computers[destination as usize].input.push(y);
                }
            }

            if steps_since_output > 1000 {
                if prev_y_sent == nat_y {
                    should_stop = true;
                }
                steps_since_output = 0;
                computers[0].input.push(nat_x);
                computers[0].input.push(nat_y);
                prev_y_sent = nat_y;
            }

            if should_stop {
                break;
            }
        }

        prev_y_sent
    }
}

struct PartialPacket {
    stage: u8,
    destination: i64,
    x: i64,
    y: i64,
}

impl PartialPacket {
    pub fn new(destination: i64) -> PartialPacket {
        PartialPacket {
            stage: 1,
            destination: destination,
            x: 0,
            y: 0,
        }
    }
    fn add_value(&mut self, value: i64) {
        match self.stage {
            1 => {
                self.x = value;
            }
            2 => {
                self.y = value;
            }
            _ => unreachable!(),
        }
        self.stage += 1;
    }
    fn is_complete(&self) -> bool {
        self.stage == 3
    }
}

struct PacketOutput {
    packets: HashMap<i64, PartialPacket>,
}

impl PacketOutput {
    pub fn new() -> PacketOutput {
        PacketOutput {
            packets: HashMap::new(),
        }
    }

    fn push_packet_piece(&mut self, origin: i64, value: i64) -> (i64, i64, i64) {
        if !self.packets.contains_key(&origin) {
            self.packets.insert(origin, PartialPacket::new(value));
        } else {
            let packet = self.packets.get_mut(&origin).unwrap();
            packet.add_value(value);
            if packet.is_complete() {
                let ret = (packet.destination, packet.x, packet.y);
                self.packets.remove(&origin);
                return ret;
            }
        }
        (-1, 0, 0)
    }
}
