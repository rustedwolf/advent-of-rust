use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp;

struct Raindeer {
    speed: i32,
    run_time: i32,
    rest_time: i32,
    score: u32
}
type Raindeers = Vec<Raindeer>;

fn main() {
    println!("--- Day 14: Reindeer Olympics ---");

    let mut raindeers = get_raindeers();
    let run_time: i32 = 2503;
    let mut longest_distance = 0;
    let raindeer_count = raindeers.len();
    let mut distances: Vec<i32> = vec![0; raindeer_count];
    let mut lead_distance = 0;
    let mut max_score = 0;

    for raindeer in &raindeers {
        let distance = get_raindeer_distance(&raindeer, run_time);
        longest_distance = cmp::max(longest_distance, distance);
    }

    println!("The distance that winning reindeer has traveled is {} km", longest_distance);

    for second in 1..run_time + 1 {
        for raindeer in 0..raindeer_count {
            distances[raindeer] = get_raindeer_distance(&raindeers[raindeer], second);
        }

        for index in 0..raindeer_count {
            lead_distance = cmp::max(lead_distance, distances[index]);
        }

        for index in 0..raindeer_count {
            if distances[index] == lead_distance {
                raindeers[index].score += 1;
            }
        }
    }

    for raindeer in &raindeers {
        max_score = cmp::max(max_score, raindeer.score);
    }

    println!("The winning reindeer has {} points", max_score);
}

fn get_raindeers() -> Raindeers {
    let path = Path::new("input/day_14");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let reader = BufReader::new(&input);
    let mut raindeers: Raindeers = Vec::new();

    for line in reader.lines() {        
        let info_string = line.unwrap();

        let info: Vec<&str> = info_string.split(' ').collect();
        let raindeer = Raindeer {
            speed: info[3].parse::<i32>().unwrap(),
            run_time: info[6].parse::<i32>().unwrap(),
            rest_time: info[13].parse::<i32>().unwrap(),
            score: 0
        };
        raindeers.push(raindeer);
    }

    raindeers
}

fn get_raindeer_distance(raindeer: &Raindeer, time: i32) -> i32 {
    let mut distance = 0;
    let mut time_remaining = time;

    while time_remaining > 0 {
        if time_remaining > raindeer.run_time {
            distance += raindeer.speed * raindeer.run_time;
        } else {
            distance += raindeer.speed * time_remaining;
        }
        time_remaining -= raindeer.run_time;
        time_remaining -= raindeer.rest_time;
    }

    distance
}