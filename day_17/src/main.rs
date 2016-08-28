use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Containers = Vec<u32>;
type Combinations = Vec<Vec<u32>>;

fn main() {
    println!("--- Day 17: No Such Thing as Too Much ---");

    let liter_count = 150;
    let containers = get_containers();
    let combinations: Combinations = get_available_combinations(liter_count, &containers);
    let minimal_combinations = get_minimal_combinations(&combinations);

    println!("Available container combination count is {}", combinations.len());
    println!("The minimum container count will be {}", minimal_combinations.len());
}

fn get_containers() -> Containers {
    let path = Path::new("data/input");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Coul not open input file: {}", reason),
    };

    let reader = BufReader::new(&input);
    let mut containers: Containers = Vec::new();

    for line in reader.lines() {
        let container = line.unwrap().parse::<u32>().unwrap();
        containers.push(container);
    }
    containers.sort();

    containers
}

fn get_available_combinations(capacity: u32, available_containers: &Containers) -> Combinations {
    let mut combinations: Combinations = Vec::new();
    let mut containers = available_containers.clone();

    while containers.len() > 0 {
        let container: u32 = containers.pop().unwrap();        
        if container < capacity { 
            let mut available_combinations = get_available_combinations(capacity - container, &containers);

            for combination in &mut available_combinations {
                combination.push(container);
            }
            combinations.append(&mut available_combinations);
        } else if container == capacity {
            let mut available_combinations: Combinations = vec![vec![container]];
            combinations.append(&mut available_combinations);
        } else {
            continue; 
        }        
    }

    combinations
}

fn get_minimum_container_count(combinations: &Combinations) -> usize {
    combinations.iter().min_by_key(|&x| x.len()).unwrap().len()
}

fn get_minimal_combinations(combinations: &Combinations) -> Combinations {
    let minimal_combination_length = get_minimum_container_count(&combinations);
    let mut minimal_combinations = combinations.clone();

    minimal_combinations.retain(|x| x.len() == minimal_combination_length);

    minimal_combinations
}

#[test]
fn test_combinations() {
    let containers = vec![20, 15, 10, 5, 5];
    let combinations = get_available_combinations(25, &containers);
    let min_countainer_count = get_minimum_container_count(&combinations);
    let minimal_combinations = get_minimal_combinations(&combinations);

    assert_eq!(combinations.len(), 4);
    assert_eq!(min_countainer_count, 2);
    assert_eq!(minimal_combinations.len(), 3);
}