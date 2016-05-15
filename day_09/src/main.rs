use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
struct Distance {
    point_a: String,
    point_b: String,
    value: u32
}

type Distances = Vec<Distance>;

trait PointDistances {
    fn get_shortest_distance(&self, starting_point: &str) -> Distance;
    fn pic_a_spot(&mut self) -> String;
    fn get_other_shortest_distance(&self, starting_point: &str, ignored_point: &str) -> u32;
    fn sort_distances(&mut self);
    fn remove_distances(&mut self, old_point: &str);
}

impl PointDistances for Distances {    
    fn get_shortest_distance(&self, starting_point: &str) -> Distance {
        let mut available_distances = self.clone();
        available_distances.retain(|dist|{
           dist.point_a == starting_point || dist.point_b == starting_point
        });
        available_distances.sort_distances();
        available_distances.reverse();
        available_distances.pop().unwrap()
    }

    fn pic_a_spot(&mut self) -> String {
        self.sort_distances();

        let point_a = &self[0].point_a;
        let point_b = &self[0].point_b;
        let distance_a_value = self.get_other_shortest_distance(point_a, point_b);
        let distance_b_value = self.get_other_shortest_distance(point_b, point_a);

        if distance_a_value > distance_b_value { 
            point_a.to_string() 
        } else { 
            point_b.to_string() 
        }
    }

    fn get_other_shortest_distance(&self, starting_point: &str, ignored_point: &str) -> u32 {
        let mut other_distances = self.clone();
        other_distances.retain(|dist|{
            dist.point_a != ignored_point &&
            dist.point_b != ignored_point &&
            (dist.point_a == starting_point || dist.point_b == starting_point)
        });
        other_distances.sort_distances();
        other_distances[0].value
    }

    fn sort_distances(&mut self) {
        self.sort_by(|a, b| a.value.cmp(&b.value));
    }

    fn remove_distances(&mut self, old_point: &str) {
        self.retain(|dist| {
            dist.point_a != old_point && dist.point_b != old_point
        });
    }
}

fn main() {
    println!("--- Day 9: All in a Single Night ---");

    let mut full_distance = 0;
    let mut distances = get_distances();
    let mut current_point = distances.pic_a_spot();
    let mut distance = distances.get_shortest_distance(&current_point);

    'running: loop {
        full_distance += distance.value;
        let old_point = current_point.clone();
        current_point = if current_point == distance.point_a {
            distance.point_b
        } else {
            distance.point_a
        }.clone();

        distances.remove_distances(&old_point);
        if !distances.is_empty() {
            distance = distances.get_shortest_distance(&current_point);
        } else {
            break 'running;
        }
    }

    println!("The distance of the shortest route is: {}", full_distance);
}

fn get_distances() -> Distances {
    let path = Path::new("input/day_9");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let reader = BufReader::new(&input);
    let mut distances: Distances = Vec::new();

    for line in reader.lines() {
        let info_string = line.unwrap();
        let info: Vec<&str> = info_string.split(" = ").collect();
        let places: Vec<&str> = info[0].rsplit(" to ").collect();
        let distance = Distance {
            point_a: places[0].to_string(),
            point_b: places[1].to_string(),
            value: info[1].parse::<u32>().unwrap()
        };
        distances.push(distance);
    }

    distances
}
