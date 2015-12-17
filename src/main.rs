extern crate crypto;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::cmp::min;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_smallest_number(input: &str, start_text: &str) -> i32 {

    let mut smallest_number = 0i32;
    let mut md = Md5::new();

    'search: loop {
        let input_str = input.to_string() + &smallest_number.to_string();
        md.input_str(&input_str);
        let out_str = md.result_str();

        if out_str.starts_with(&start_text) {
            break 'search;
        } else { 
            md.reset();
            smallest_number += 1;
        }
    }

    smallest_number
}

fn main() {

    // Day 1

    println!("# Day 1");

    let mut count = 0;
    let mut floor = 0;
    let mut basement_visited = false;

    let path = Path::new("resources/day1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display, Error::description(&e))
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

    // Day 2

    println!("# Day 2");

    let (mut paper_amount, mut ribbon_length) = (0i32, 0i32);

    let path2 = Path::new("resources/day2.txt");
    let display2 = path2.display();
    let file2 = match File::open(&path2) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display2, Error::description(&e))
    };

    let input_contents = BufReader::new(&file2);
    for line in input_contents.lines() {
        let box_info = line.unwrap();
        let box_sizes : Vec<&str> = box_info.rsplit('x').collect();
        let l: i32 = box_sizes[0].parse().unwrap();
        let w: i32 = box_sizes[1].parse().unwrap();
        let h: i32 = box_sizes[2].parse().unwrap();

        let side1 = l * w;
        let side2 = w * h;
        let side3 = h * l;

        let mut sides = [l, w, h];
        sides.sort();

        let smallest_side = min(&side1, min(&side2, &side3));

        paper_amount += 2 * (side1 + side2 + side3) + smallest_side;
        ribbon_length += sides[0] * 2 + sides[1] * 2 + l * w * h;
    }

    println!("The required square feet of paper is {}", paper_amount);
    println!("Elves also neet {} feet of ribbon", ribbon_length);


    // Day 3 and I still haven't splitted the code into chunks
    // ... will do that tomorrow
    println!("# Day 3");

    let path3 = Path::new("resources/day3.txt");
    let display3 = path3.display();
    let mut file3 = match File::open(&path3) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display3, Error::description(&e))
    };

    let mut visited_houses: Vec<(i32, i32)> = Vec::new();
    let mut santa_position = (0, 0);
    let mut robo_position = (0, 0);
    visited_houses.push(santa_position);

    let mut directions2 = String::new();
    match file3.read_to_string(&mut directions2) {
        Err(e) => panic!("Couldn't read {}: {}", display, Error::description(&e)),
        Ok(_) => {}
    }

    for d in directions2.chars() {
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

    for d in directions2.chars() {
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

    // Day 4
    println!("# Day 4");
    
    let input = "iwrupvqb";
    let mut smallest_number = get_smallest_number(&input, "00000");

    println!("The smallest number for 5 leading zero hash is: {:?}", smallest_number);

    smallest_number = get_smallest_number(&input, "000000");
      
    println!("The smallest number for 6 leading zero hash is: {:?}", smallest_number);
}
