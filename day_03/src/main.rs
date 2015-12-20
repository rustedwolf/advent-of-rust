use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

fn main() {

    println!("--- Day 3: Perfectly Spherical Houses in a Vacuum ---");

    let path = Path::new("input/day3.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display, Error::description(&e))
    };

    let mut visited_houses: Vec<(i32, i32)> = Vec::new();
    let mut santa_position = (0, 0);
    let mut robo_position = (0, 0);
    visited_houses.push(santa_position);

    let mut directions = String::new();
    match file.read_to_string(&mut directions) {
        Err(e) => panic!("Couldn't read {}: {}", display, Error::description(&e)),
        Ok(_) => {}
    }

    for d in directions.chars() {
        match d {
            '^' => {santa_position.1 += 1},
            'v' => {santa_position.1 -= 1},
            '>' => {santa_position.0 += 1},
            '<' => {santa_position.0 -= 1},
            _ => panic!("Bad instruction input")
        }
        visited_houses.push(santa_position);
    }
    visited_houses.sort();
    visited_houses.dedup();
    println!("Santa delivered presents to {} houses", visited_houses.len());

    santa_position = (0, 0);
    visited_houses.clear();
    visited_houses.push(santa_position);

    const SANTAS_TURN: bool = true;
    const ROBOS_TURN: bool = false;

    let mut turn = SANTAS_TURN;

    for d in directions.chars() {
        let position = if turn == SANTAS_TURN { &mut santa_position } else { &mut robo_position };

        match d {
            '^' => {position.1 += 1},
            'v' => {position.1 -= 1},
            '>' => {position.0 += 1},
            '<' => {position.0 -= 1},
            _ => panic!("Bad instruction input")
        }

        visited_houses.push(*position);

        turn = if turn == SANTAS_TURN { ROBOS_TURN } else { SANTAS_TURN }
    }
    visited_houses.sort();
    visited_houses.dedup();
    println!("With Robo-Santa they delivered presents to {} houses", visited_houses.len());
}
