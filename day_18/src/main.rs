use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Lights = Vec<Vec<bool>>;

fn main() {
    println!("--- Day 18: Like a GIF For Your Yard ---");

    let mut lights = get_lights();
    let mut broken_lights = lights.clone();
    let step_count = 100;

    for _ in 0..step_count {
        lights = toggle_lights_state(&lights);
    }

    println!("After {} steps there is {} lights that are on", step_count, count_shining_lights(&lights));

    let last_row = broken_lights.len() - 1;
    let row_last_pos = broken_lights[0].len() - 1;

    lights_bug(&mut broken_lights, last_row, row_last_pos);

    for _ in 0..step_count {
        broken_lights = toggle_lights_state(&broken_lights);
        lights_bug(&mut broken_lights, last_row, row_last_pos);
    }

    println!("While lights with stucked sockets have {}", count_shining_lights(&broken_lights));
}

fn get_lights() -> Lights {
    let path = Path::new("data/input");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Coul not open input file: {}", reason),
    };
    let reader = BufReader::new(&input);
    let mut lights = Vec::new();

    for line in reader.lines() {
        let row_of_lights: Vec<bool> = line.unwrap().chars().map(|x| x == '#').collect();
        lights.push(row_of_lights);
    }

    lights
}

fn toggle_lights_state(lights: &Lights) -> Lights {
    let mut new_state: Lights = lights.clone();

    for r in 0..lights.len() {
        for l in 0..lights[r].len() {
            new_state[r][l] = get_next_light_state(&lights, r, l);
        }
    }

    new_state
}

fn get_next_light_state(lights: &Lights, row: usize, col: usize) -> bool {
    let neighbors_on = count_shining_neighbours(&lights, row, col);

    if lights[row][col] {
        neighbors_on == 2 || neighbors_on == 3
    } else {
        neighbors_on == 3
    }
}

fn count_shining_neighbours(lights: &Lights, row: usize, col: usize) -> usize {
    let mut shining_lights_count: usize = 0;
    let row_length = lights.len();

    for r in if row == 0 { 0 } else { row - 1 }..row + 2 {
        if r == row_length { continue; }
        let col_length = lights[r].len();

        for l in if col == 0 { 0 } else { col - 1 }..col + 2 {
            if (r == row && l == col) || l == col_length { continue; }
            if lights[r][l] { shining_lights_count += 1; }
        }
    }

    shining_lights_count
}

fn count_shining_lights(lights: &Lights) -> usize {
    let mut shining_lights_count: usize = 0;

    for r in 0..lights.len() {
        for l in 0..lights[r].len() {
            if lights[r][l] { shining_lights_count += 1; }
        }
    }

    shining_lights_count
}

/**
 * Causes lights at the grid corners to be stuck "on"
 */
fn lights_bug(lights: &mut Lights, last_row: usize, row_last_pos: usize) {
    lights[0][0] = true;
    lights[0][row_last_pos] = true;
    lights[last_row][0] = true;
    lights[last_row][row_last_pos] = true;
}