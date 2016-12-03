use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    println!("--- Day 2: Bathroom Security ---");

    let input = get_input().unwrap();
    let instructions = parse_input(&input);
    let keypad: Vec<Vec<char>> = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9']
    ];
    let real_keypad: Vec<Vec<char>> = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];

    println!("Bathroom code should be {}", get_code(&instructions, &keypad, (1, 1)));
    println!("But the real code is {}", get_code(&instructions, &real_keypad, (2, 0)));
}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let mut instructions: Vec<Vec<char>> = Vec::new();

    for instruction in input.lines() {
        instructions.push(instruction.chars().collect());
    }

    instructions
}

fn get_code(instructions: &Vec<Vec<char>>, keypad: &Vec<Vec<char>>, starting_position: (usize, usize)) -> String {
    let mut key_position = starting_position;
    let mut code = "".to_string();

    for key_instructions in instructions {
        code.push(get_digit(&key_instructions, &mut key_position, &keypad));
    }

    code
}

fn get_digit(key_instructions: &Vec<char>, key_position: &mut (usize, usize), keypad: &Vec<Vec<char>>) -> char {
    for instruction in key_instructions {
        match instruction {
            &'U' => if key_position.1 > 0
                && can_move((key_position.0, key_position.1 - 1), keypad) {
                    key_position.1 -= 1;
                },
            &'D' => if key_position.1 < keypad.len() - 1
                && can_move((key_position.0, key_position.1 + 1), keypad) {
                    key_position.1 += 1;
                },
            &'L' => if key_position.0 > 0
                && can_move((key_position.0 - 1, key_position.1), keypad) {
                    key_position.0 -= 1;
                },
            &'R' => if key_position.0 < keypad[key_position.1].len() - 1
                && can_move((key_position.0 + 1, key_position.1), keypad) {
                    key_position.0 += 1;
                },
            _ => {}
        }
    }

    keypad[key_position.1][key_position.0]
}

fn can_move(key_position: (usize, usize), keypad: &Vec<Vec<char>>) -> bool {
    keypad[key_position.1][key_position.0] != ' '
}