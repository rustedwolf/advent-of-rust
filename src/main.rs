use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

fn main() {

    // Day 1

    println!("# Day 1");

    let mut count = 0;
    let mut floor = 0;
    let mut basement_visited = false;

    let path = Path::new("resources/day1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}", display, Error::description(&e)),                
        Ok(file) => file
    };

    let mut directions = String::new();
    match file.read_to_string(&mut directions) {
        Err(e) => panic!("Couldn't read {}: {}", display, Error::description(&e)),
        Ok(_) => {}
    }

    for x in directions.chars() {
        count += 1;
        floor += match x {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if !basement_visited && floor == -1 {
            basement_visited = true;
            println!("Santa entered the basement on {}'th direction", count);   
        }
    }

    println!("The instructions will take Santa to {:?} floor", floor);
}
