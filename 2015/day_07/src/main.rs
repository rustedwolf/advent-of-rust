use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

type Wires = HashMap<String, String>;
type Signals = HashMap<String, u16>;

struct Circuit {
    instructions: Wires,
    signals: Signals,
}

trait Wire {
    fn get_signal(&mut self, wire_name: &str) -> u16;
    fn parse_signal(&mut self, wire_name: &str) -> u16;
}

impl Wire for Circuit {
    fn get_signal(&mut self, wire_name: &str) -> u16 {
        if self.signals.contains_key(wire_name) {
            self.signals[wire_name]
        } else {
            self.parse_signal(wire_name)
        }
    }

    fn parse_signal(&mut self, wire_name: &str) -> u16 {
        let number = wire_name.parse::<u16>();
        if !number.is_err() {
            return number.unwrap();
        }

        let gate_instruction = match self.instructions.get(wire_name) {
            Some(instruction) => instruction.clone(),
            None => panic!("Instructions for wire {:?} are missing", wire_name),
        };
        let segments: Vec<&str> = gate_instruction.split_whitespace().collect();

        let signal = match segments.len() {
            3 => {
                let left_val = self.get_signal(&segments[0]);
                let right_val = self.get_signal(&segments[2]);
                let command = segments[1];
                match command {
                    "OR" => {
                        left_val | right_val
                    },
                    "AND" => {
                        left_val & right_val
                    },
                    "RSHIFT" => {
                        left_val >> right_val
                    },
                    "LSHIFT" => {
                        left_val << right_val
                    },
                    _ => panic!("No instructions for command {:?}", command),
                }
            },
            2 => {
                let command = segments[0];
                let input = self.get_signal(&segments[1]);
                match command {
                    "NOT" => {
                        !input
                    },
                    _ => panic!("No instructions for command {:?}", command),
                }
            },
            1 => self.get_signal(&segments[0]),
            _ => panic!("Something's wrong with the instructions"),
        };

        self.signals.insert(wire_name.to_string(), signal);

        self.signals[wire_name]
    }
}

fn main() {

    println!("--- Day 7: Some Assembly Required ---");

    let mut circuit = Circuit {
        instructions: Wires::new(),
        signals: Signals::new()
    };

    let path = Path::new("input/day_7.txt");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let buffer = BufReader::new(&input);

    for line in buffer.lines() {
        let instruction = line.unwrap();
        let instruction_parts: Vec<&str> = instruction.rsplit(" -> ").collect();

        circuit.instructions.insert(instruction_parts[0].to_string(), instruction_parts[1].to_string());
    }

    let mut signal = circuit.get_signal(&"a");

    println!("'a' wire's signal is {}", signal);

    circuit.signals.clear();
    circuit.signals.insert("b".to_string(), signal);

    signal = circuit.get_signal(&"a");

    println!("After overriding 'b' wire's signal, 'a' wire's signal is {}", signal);
}

#[test]
fn test_get_signal() {
    let mut circuit = Circuit {
        instructions: Wires::new(),
        signals: Signals::new()
    };

    circuit.instructions.insert("d".to_string(), "x AND y".to_string());
    circuit.instructions.insert("e".to_string(), "x OR y".to_string());
    circuit.instructions.insert("f".to_string(), "x LSHIFT 2".to_string());
    circuit.instructions.insert("g".to_string(), "y RSHIFT 2".to_string());
    circuit.instructions.insert("h".to_string(), "NOT x".to_string());
    circuit.instructions.insert("i".to_string(), "NOT y".to_string());
    circuit.instructions.insert("x".to_string(), "123".to_string());
    circuit.instructions.insert("y".to_string(), "456".to_string());

    assert_eq!(circuit.get_signal(&"d"), 72u16);
    assert_eq!(circuit.get_signal(&"e"), 507u16);
    assert_eq!(circuit.get_signal(&"f"), 492u16);
    assert_eq!(circuit.get_signal(&"g"), 114u16);
    assert_eq!(circuit.get_signal(&"h"), 65412u16);
    assert_eq!(circuit.get_signal(&"i"), 65079u16);
    assert_eq!(circuit.get_signal(&"x"), 123u16);
    assert_eq!(circuit.get_signal(&"y"), 456u16);
}