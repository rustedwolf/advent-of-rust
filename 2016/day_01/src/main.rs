use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

enum Side {
    Right,
    Left,
}

struct Direction {
    turn: Side,
    blocks: i32,
}

fn main() {
    println!("--- Day 1: No Time for a Taxicab ---");

    let input = get_input().unwrap();
    let directions = get_directions(&input);
    let mut location = (0, 0);
    let mut orientation = 0;
    let mut route = vec![(0,0)];

    for direction in directions {
        orientation = match direction.turn {
            Side::Right => (orientation + 1) % 4,
            Side::Left => if orientation == 0 { 3 } else {
                orientation - 1
            },
        };
        move_by(&mut location, direction.blocks, orientation, &mut route);
    }

    let blocks_to_location = get_blocks_distance(location);
    let blocks_to_real_location = get_blocks_distance(get_first_twise_visited_location(&route));

    println!("The headquaters should {} blocks away", blocks_to_location);
    println!("But the it's really {} blocks away", blocks_to_real_location);
}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn get_directions(input: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    let infos: Vec<&str> = input.lines().next().unwrap().split(", ").collect();

    for info in infos {
        let (side, blocks) = info.split_at(1);
        directions.push(Direction {
            turn: match side.chars().nth(0) {
                Some('R') => Side::Right,
                Some('L') => Side::Left,
                _ => panic!("Unknown side in instructions")
            },
            blocks: match blocks.parse::<i32>() {
                Ok(number) => number,
                Err(reason) => panic!("Could not parse block numbers: {} \n input: {:?}", reason, blocks),
            },
        });
    }

    directions
}

fn move_by(location: &mut (i32, i32), block_count: i32, orientation: usize, route: &mut Vec<(i32, i32)>) {
    for _ in 0..block_count {
        match orientation {
            0 => location.1 += 1,
            1 => location.0 += 1,
            2 => location.1 -= 1,
            3 => location.0 -= 1,
            _ => {},
        }
        route.push(location.clone());
    }
}

fn get_blocks_distance(location: (i32, i32)) -> i32 {
    location.0.abs() + location.1.abs()
}

fn get_first_twise_visited_location(route: &Vec<(i32, i32)>) -> (i32, i32) {
    let mut twise_visited_location = (0, 0);
    for (index, location) in route.iter().enumerate() {
        if index == 0 { continue; }

        let visited_locations = route.windows(index).next().unwrap();

        if visited_locations.contains(location) {
            twise_visited_location = location.clone();
            break;
        }
    }

    twise_visited_location
}