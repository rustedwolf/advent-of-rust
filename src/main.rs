use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::cmp::min;

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
    let display = path2.display();    
    let file2 = match File::open(&path2) {               
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display, Error::description(&e))
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
}
