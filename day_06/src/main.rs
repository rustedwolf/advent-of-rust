use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Lights = Vec<Vec<bool>>;
type BrightLights = Vec<Vec<u32>>;

fn switch_ligh(position: &(usize, usize), command: &str, lights: &mut Lights) {
    lights[position.0][position.1] = match command {
        "on" => true,
        "off" => false,
        "toggle" => !lights[position.0][position.1],
        _ => panic!("Wrong command input!")
    };
}

fn switch_brightness(position: &(usize, usize), command: &str, bright_lights: &mut BrightLights) {
    let brightness_level = bright_lights[position.0][position.1];
    
    bright_lights[position.0][position.1] = match command {
        "on" => brightness_level + 1,
        "off" => if brightness_level == 0 { 0 } else { brightness_level - 1 },
        "toggle" => brightness_level + 2,
        _ => panic!("Wrong command input!")
    };
}

fn main() {

    println!("--- Day 6: Probably a Fire Hazard ---");

    let mut lights: Lights = vec![vec![false; 1000]; 1000];
    let mut bright_lights: BrightLights = vec![vec![0; 1000]; 1000];
    let mut lights_on = 0;
    let mut brightness = 0u64;

    let path = Path::new("input/day_6.txt");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let instructions = BufReader::new(&input);

    for line in instructions.lines() {
        let details = line.unwrap();
        let mut instruction: Vec<&str> = details.split_whitespace().collect();
        let to = instruction.pop().unwrap();
        instruction.pop();
        let from = instruction.pop().unwrap();
        let start_pos: Vec<&str> = from.split(',').collect();
        let end_pos: Vec<&str> = to.split(',').collect();

        let command = match instruction[0] {
            "turn" => {
                &instruction[1]
            },
            "toggle" => {
                &instruction[0]
            },
            _ => panic!("Invalid command provided!"),
        };

        let start_x = start_pos[0].parse::<usize>().unwrap();
        let start_y = start_pos[1].parse::<usize>().unwrap();
        let end_x = end_pos[0].parse::<usize>().unwrap();
        let end_y = end_pos[1].parse::<usize>().unwrap();

        for x in start_x..(end_x + 1) {
            for y in start_y..(end_y + 1) {
                switch_ligh(&(x, y), command, &mut lights);
                switch_brightness(&(x, y), command, &mut bright_lights);
            } 
        }
    }

    for row in lights {
        for col in row {
            if col {
                lights_on += 1;
            }
        }
    }

    for row in bright_lights {
        for col in row {
            brightness += col as u64;
        }
    }    

    println!("There are {} lights on", lights_on);
    println!("There brightness level is {}", brightness);
}